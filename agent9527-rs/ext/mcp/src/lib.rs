use agent9527_core::config::Config;
use agent9527_extension_api::ExtensionFuture;
use agent9527_extension_api::ExtensionRegistryBuilder;
use agent9527_extension_api::McpServerContribution;
use agent9527_extension_api::McpServerContributionContext;
use agent9527_extension_api::McpServerContributor;
use agent9527_mcp::AGENT9527_APPS_MCP_SERVER_NAME;
use agent9527_mcp::hosted_plugin_runtime_mcp_server_config;

mod executor_plugin;

struct HostedPluginRuntimeExtension;

impl McpServerContributor<Config> for HostedPluginRuntimeExtension {
    fn id(&self) -> &'static str {
        "hosted_plugin_runtime"
    }

    fn contribute<'a>(
        &'a self,
        context: McpServerContributionContext<'a, Config>,
    ) -> ExtensionFuture<'a, Vec<McpServerContribution>> {
        Box::pin(async move {
            let config = context.config();
            let name = AGENT9527_APPS_MCP_SERVER_NAME.to_string();
            if !config.features.enabled(agent9527_features::Feature::Apps) {
                return vec![McpServerContribution::Remove { name }];
            }

            vec![McpServerContribution::Set {
                name,
                config: Box::new(hosted_plugin_runtime_mcp_server_config(
                    &config.chatgpt_base_url,
                    config.apps_mcp_product_sku.as_deref(),
                    context.originator(),
                )),
            }]
        })
    }
}

pub fn install(builder: &mut ExtensionRegistryBuilder<Config>) {
    builder.mcp_server_contributor(std::sync::Arc::new(HostedPluginRuntimeExtension));
}

/// Installs discovery for MCP servers declared by thread-selected executor plugins.
pub fn install_executor_plugins(
    builder: &mut ExtensionRegistryBuilder<Config>,
    environment_manager: std::sync::Arc<agent9527_exec_server::EnvironmentManager>,
) {
    builder.mcp_server_contributor(std::sync::Arc::new(
        executor_plugin::SelectedExecutorPluginMcpContributor::new(environment_manager),
    ));
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
