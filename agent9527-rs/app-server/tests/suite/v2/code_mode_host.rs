use std::net::TcpListener;
use std::time::Duration;

use agent9527_app_server_protocol::ThreadStartParams;
use agent9527_app_server_protocol::TurnStartParams;
use agent9527_app_server_protocol::TurnStatus;
use agent9527_app_server_protocol::UserInput;
use agent9527_features::Feature;
use anyhow::Result;
use app_test_support::MockResponsesConfig;
use app_test_support::TestAppServer;
use core_test_support::responses;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::TempDir;
use tokio::time::timeout;

#[cfg(any(target_os = "macos", windows))]
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(/*secs*/ 60);
#[cfg(not(any(target_os = "macos", windows)))]
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(/*secs*/ 10);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn app_server_shares_flag_selected_code_mode_host_across_threads() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let websocket_url = format!("ws://{}", listener.local_addr()?);
    drop(listener);
    let host_url = websocket_url.clone();
    let websocket_host =
        tokio::spawn(async move { agent9527_code_mode_host::run_main(&host_url).await });
    let readiness_url = format!(
        "http://{}/readyz",
        websocket_url
            .strip_prefix("ws://")
            .expect("websocket URL should have ws scheme")
    );
    timeout(DEFAULT_READ_TIMEOUT, async {
        loop {
            if let Ok(response) = reqwest::get(&readiness_url).await
                && response.status().is_success()
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await?;

    let model_server = responses::start_mock_server().await;
    let response_mock = responses::mount_sse_sequence(
        &model_server,
        vec![
            responses::sse(vec![
                responses::ev_response_created("resp-1"),
                responses::ev_custom_tool_call(
                    "first-remote-cell",
                    "exec",
                    "text('remote app-server host')",
                ),
                responses::ev_completed("resp-1"),
            ]),
            responses::sse(vec![
                responses::ev_assistant_message("msg-1", "Done"),
                responses::ev_completed("resp-2"),
            ]),
            responses::sse(vec![
                responses::ev_response_created("resp-3"),
                responses::ev_custom_tool_call(
                    "second-remote-cell",
                    "exec",
                    "text('remote app-server host')",
                ),
                responses::ev_completed("resp-3"),
            ]),
            responses::sse(vec![
                responses::ev_assistant_message("msg-2", "Done"),
                responses::ev_completed("resp-4"),
            ]),
        ],
    )
    .await;

    let agent9527_home = TempDir::new()?;
    MockResponsesConfig::new(&model_server.uri())
        .enable_feature(Feature::CodeModeOnly)
        .write(agent9527_home.path())?;
    let original_config = std::fs::read_to_string(agent9527_home.path().join("config.toml"))?;
    let mut app_server = TestAppServer::builder()
        .with_agent9527_home(agent9527_home.path())
        .with_args(&["--code-mode-host", &websocket_url])
        .build_initialized_with_timeout(DEFAULT_READ_TIMEOUT)
        .await?;

    for prompt in ["run the first remote cell", "run the second remote cell"] {
        let thread = app_server
            .start_thread(ThreadStartParams::default())
            .await?;
        let completed = timeout(
            DEFAULT_READ_TIMEOUT,
            app_server.start_turn_and_wait_for_completion(TurnStartParams {
                thread_id: thread.thread.id,
                input: vec![UserInput::Text {
                    text: prompt.to_string(),
                    text_elements: Vec::new(),
                }],
                ..Default::default()
            }),
        )
        .await??;

        assert_eq!(completed.turn.status, TurnStatus::Completed);
    }

    let requests = response_mock.requests();
    assert_eq!(requests.len(), 4);
    for (request, call_id) in [
        (&requests[1], "first-remote-cell"),
        (&requests[3], "second-remote-cell"),
    ] {
        let output = request.custom_tool_call_output(call_id);
        assert_eq!(
            output["output"]
                .as_array()
                .and_then(|items| items.last())
                .cloned(),
            Some(json!({
                "type": "input_text",
                "text": "remote app-server host",
            }))
        );
    }
    assert_eq!(
        std::fs::read_to_string(agent9527_home.path().join("config.toml"))?,
        original_config
    );

    websocket_host.abort();
    let _ = websocket_host.await;
    Ok(())
}
