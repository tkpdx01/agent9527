use super::*;
use agent9527_config::McpServerTransportConfig;
use agent9527_core::config::ConfigBuilder;
use agent9527_extension_api::ExtensionData;
use agent9527_extension_api::ExtensionDataInit;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn hosted_plugin_runtime_forwards_thread_originator() -> Result<(), Box<dyn std::error::Error>>
{
    let agent9527_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .agent9527_home(agent9527_home.path().to_path_buf())
        .fallback_cwd(Some(agent9527_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            ("chatgpt_base_url".to_string(), "https://chatgpt.com".into()),
        ])
        .build()
        .await?;
    let thread_init = ExtensionDataInit::new();
    let thread_store = ExtensionData::new("thread");

    let contributions = HostedPluginRuntimeExtension
        .contribute(McpServerContributionContext::for_step(
            &config,
            &thread_init,
            &thread_store,
            "agent9527_work_desktop",
            /*ready_selected_capability_roots*/ &[],
            /*executor_capability_discovery*/ None,
        ))
        .await;
    let [McpServerContribution::Set { config: server, .. }] = contributions.as_slice() else {
        panic!("hosted plugin runtime should contribute one server");
    };
    let McpServerTransportConfig::StreamableHttp { http_headers, .. } = &server.transport else {
        panic!("hosted plugin runtime should use streamable HTTP");
    };

    assert_eq!(
        http_headers
            .as_ref()
            .and_then(|headers| headers.get("originator")),
        Some(&"agent9527_work_desktop".to_string())
    );

    Ok(())
}
