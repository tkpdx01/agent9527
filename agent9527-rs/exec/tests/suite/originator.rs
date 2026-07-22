#![cfg(not(target_os = "windows"))]
#![allow(clippy::unwrap_used)]

use agent9527_login::default_client::AGENT9527_INTERNAL_ORIGINATOR_OVERRIDE_ENV_VAR;
use core_test_support::responses;
use core_test_support::test_agent9527_exec::test_agent9527_exec;
use wiremock::matchers::header;

/// Verify that when the server reports an error, `agent9527-exec` exits with a
/// non-zero status code so automation can detect failures.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn send_agent9527_exec_originator() -> anyhow::Result<()> {
    let test = test_agent9527_exec();

    let server = responses::start_mock_server().await;
    let body = responses::sse(vec![
        responses::ev_response_created("response_1"),
        responses::ev_assistant_message("response_1", "Hello, world!"),
        responses::ev_completed("response_1"),
    ]);
    responses::mount_sse_once_match(&server, header("Originator", "agent9527_exec"), body).await;

    test.cmd_with_server(&server)
        .env_remove(AGENT9527_INTERNAL_ORIGINATOR_OVERRIDE_ENV_VAR)
        .arg("--skip-git-repo-check")
        .arg("tell me something")
        .assert()
        .code(0);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn supports_originator_override() -> anyhow::Result<()> {
    let test = test_agent9527_exec();

    let server = responses::start_mock_server().await;
    let body = responses::sse(vec![
        responses::ev_response_created("response_1"),
        responses::ev_assistant_message("response_1", "Hello, world!"),
        responses::ev_completed("response_1"),
    ]);
    responses::mount_sse_once_match(
        &server,
        header("Originator", "agent9527_exec_override"),
        body,
    )
    .await;

    test.cmd_with_server(&server)
        .env(
            "AGENT9527_INTERNAL_ORIGINATOR_OVERRIDE",
            "agent9527_exec_override",
        )
        .arg("--skip-git-repo-check")
        .arg("tell me something")
        .assert()
        .code(0);

    Ok(())
}
