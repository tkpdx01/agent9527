#![allow(clippy::unwrap_used)]

use agent9527_config::McpServerTransportConfig;
use agent9527_config::types::OAuthCredentialsStoreMode;
use agent9527_core::config::Constrained;
use agent9527_login::Agent9527Auth;
use agent9527_login::AuthKeyringBackendKind;
use agent9527_login::AuthManager;
use agent9527_login::ExternalAuth;
use agent9527_login::ExternalAuthFuture;
use agent9527_login::ExternalAuthRefreshContext;
use agent9527_mcp::AGENT9527_APPS_MCP_SERVER_NAME;
use agent9527_mcp::Agent9527AppsToolsCache;
use agent9527_mcp::EffectiveMcpServer;
use agent9527_mcp::ElicitationRequestRouter;
use agent9527_mcp::McpConnectionManager;
use agent9527_mcp::McpRuntimeContext;
use agent9527_mcp::McpToolCatalogCache;
use agent9527_mcp::ToolPluginProvenance;
use agent9527_protocol::models::PermissionProfile;
use agent9527_protocol::protocol::AskForApproval;
use anyhow::Result;
use core_test_support::apps_test_server::AppsTestServer;
use core_test_support::responses::start_mock_server;
use core_test_support::skip_if_no_network;
use pretty_assertions::assert_eq;
use rmcp::model::ElicitationCapability;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

// Installs a known snapshot through AuthManager's public external-auth path.
struct StaticExternalAuth(Agent9527Auth);

impl ExternalAuth for StaticExternalAuth {
    fn resolve(&self) -> ExternalAuthFuture<'_, Agent9527Auth> {
        Box::pin(async { Ok(self.0.clone()) })
    }

    fn refresh(
        &self,
        _context: ExternalAuthRefreshContext,
    ) -> ExternalAuthFuture<'_, Agent9527Auth> {
        Box::pin(async { Ok(self.0.clone()) })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hosted_plugin_runtime_ps_mcp_tool_calls_use_current_auth_manager_token() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let apps_server = AppsTestServer::mount_hosted_plugin_runtime_searchable(&server).await?;
    let home = Arc::new(TempDir::new()?);
    let expected_auth = Agent9527Auth::from_external_chatgpt_tokens(
        "header.e30.first",
        "test-account",
        /*chatgpt_plan_type*/ None,
    )?;
    let auth_manager = AuthManager::from_auth_for_testing_with_home(
        expected_auth.clone(),
        home.path().to_path_buf(),
    );
    // Build the hosted-plugin config directly so the local test origin can
    // exercise the connection-manager auth path. Effective server resolution
    // correctly strips ChatGPT auth from untrusted localhost origins.
    let mut hosted_plugin_runtime_config = agent9527_mcp::hosted_plugin_runtime_mcp_server_config(
        &apps_server.chatgpt_base_url,
        /*apps_mcp_product_sku*/ None,
        /*originator*/ None,
    );
    let McpServerTransportConfig::StreamableHttp {
        bearer_token_env_var,
        ..
    } = &mut hosted_plugin_runtime_config.transport
    else {
        panic!("hosted plugin runtime should use streamable HTTP");
    };
    // Keep the test on the AuthManager path even if the developer has the
    // debug bearer override in their environment.
    *bearer_token_env_var = None;
    let mcp_servers = HashMap::from([(
        AGENT9527_APPS_MCP_SERVER_NAME.to_string(),
        EffectiveMcpServer::configured(hosted_plugin_runtime_config),
    )]);
    let (tx_event, rx_event) = async_channel::unbounded();
    drop(rx_event);
    let approval_policy = Constrained::allow_any(AskForApproval::Never);
    let manager = McpConnectionManager::new(
        &mcp_servers,
        OAuthCredentialsStoreMode::default(),
        AuthKeyringBackendKind::default(),
        &approval_policy,
        "test".to_string(),
        tx_event,
        CancellationToken::new(),
        PermissionProfile::default(),
        McpRuntimeContext::new(
            Arc::new(agent9527_exec_server::EnvironmentManager::without_environments()),
            home.path().to_path_buf(),
        ),
        home.path().to_path_buf(),
        Agent9527AppsToolsCache::default(),
        McpToolCatalogCache::default(),
        agent9527_mcp::agent9527_apps_tools_cache_key(Some(&expected_auth)),
        /*prefix_mcp_tool_names*/ true,
        ElicitationCapability::default(),
        /*supports_openai_form_elicitation*/ false,
        ToolPluginProvenance::default(),
        Some(&expected_auth),
        Some(Arc::clone(&auth_manager)),
        /*elicitation_reviewer*/ None,
        /*elicitation_lifecycle*/ None,
        ElicitationRequestRouter::default(),
    )
    .await;
    // The model-provider test covers AuthManager reload behavior. Keep this
    // regression focused on core MCP wiring by updating the same shared
    // manager after the MCP client has been created.
    auth_manager
        .set_external_auth(Arc::new(StaticExternalAuth(
            Agent9527Auth::from_external_chatgpt_tokens(
                "header.e30.reloaded",
                "test-account",
                /*chatgpt_plan_type*/ None,
            )?,
        )))
        .await?;

    // The manager and its static fallback were created before the auth update,
    // so this tool call only sees the new token if the Agent9527 Apps provider
    // reads the shared AuthManager at request time.
    let tool_result = manager
        .call_tool(
            AGENT9527_APPS_MCP_SERVER_NAME,
            "calendar_create_event",
            Some(json!({
                "title": "Lunch",
                "starts_at": "2026-06-18T12:00:00Z",
            })),
            /*meta*/ None,
        )
        .await?;
    assert_eq!(tool_result.is_error, Some(false));

    let requests = server
        .received_requests()
        .await
        .expect("mock server should capture tool-call requests");
    let tool_call_request = requests
        .iter()
        .find(|request| {
            request.url.path() == "/api/agent9527/ps/mcp"
                && serde_json::from_slice::<Value>(&request.body)
                    .ok()
                    .is_some_and(|body| {
                        body.get("method").and_then(Value::as_str) == Some("tools/call")
                    })
        })
        .expect("Agent9527 Apps should receive a tool call");
    assert_eq!(
        tool_call_request
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok()),
        Some("Bearer header.e30.reloaded")
    );

    Ok(())
}
