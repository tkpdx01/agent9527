use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use agent9527_analytics::AnalyticsEventsClient;
use agent9527_analytics::AnalyticsJsonRpcError;
use agent9527_analytics::InputError;
use agent9527_analytics::TurnSteerRequestError;
use agent9527_app_server_protocol::Account;
use agent9527_app_server_protocol::AccountLoginCompletedNotification;
use agent9527_app_server_protocol::AccountTokenUsageDailyBucket;
use agent9527_app_server_protocol::AccountTokenUsageSummary;
use agent9527_app_server_protocol::AccountUpdatedNotification;
use agent9527_app_server_protocol::AddCreditsNudgeCreditType;
use agent9527_app_server_protocol::AddCreditsNudgeEmailStatus;
use agent9527_app_server_protocol::AdditionalContextEntry;
use agent9527_app_server_protocol::AdditionalContextKind;
use agent9527_app_server_protocol::Agent9527ErrorInfo;
use agent9527_app_server_protocol::AppListUpdatedNotification;
use agent9527_app_server_protocol::AppSummary;
use agent9527_app_server_protocol::AppTemplateSummary;
use agent9527_app_server_protocol::AppTemplateUnavailableReason;
use agent9527_app_server_protocol::AppsInstalledParams;
use agent9527_app_server_protocol::AppsInstalledResponse;
use agent9527_app_server_protocol::AppsListParams;
use agent9527_app_server_protocol::AppsListResponse;
use agent9527_app_server_protocol::AppsReadParams;
use agent9527_app_server_protocol::AppsReadResponse;
use agent9527_app_server_protocol::AskForApproval;
use agent9527_app_server_protocol::AuthMode;
use agent9527_app_server_protocol::CancelLoginAccountParams;
use agent9527_app_server_protocol::CancelLoginAccountResponse;
use agent9527_app_server_protocol::CancelLoginAccountStatus;
use agent9527_app_server_protocol::ClientInfo;
use agent9527_app_server_protocol::ClientRequest;
use agent9527_app_server_protocol::ClientResponsePayload;
use agent9527_app_server_protocol::CollaborationModeListParams;
use agent9527_app_server_protocol::CollaborationModeListResponse;
use agent9527_app_server_protocol::CommandExecParams;
use agent9527_app_server_protocol::CommandExecResizeParams;
use agent9527_app_server_protocol::CommandExecTerminateParams;
use agent9527_app_server_protocol::CommandExecWriteParams;
use agent9527_app_server_protocol::ConfigWarningNotification;
use agent9527_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use agent9527_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use agent9527_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use agent9527_app_server_protocol::ConversationGitInfo;
use agent9527_app_server_protocol::ConversationSummary;
use agent9527_app_server_protocol::DeprecationNoticeNotification;
use agent9527_app_server_protocol::DynamicToolFunctionSpec;
use agent9527_app_server_protocol::DynamicToolNamespaceTool;
use agent9527_app_server_protocol::DynamicToolSpec;
use agent9527_app_server_protocol::EnvironmentAddParams;
use agent9527_app_server_protocol::EnvironmentAddResponse;
use agent9527_app_server_protocol::EnvironmentInfoParams;
use agent9527_app_server_protocol::EnvironmentInfoResponse;
use agent9527_app_server_protocol::EnvironmentShellInfo;
use agent9527_app_server_protocol::EnvironmentStatusKind;
use agent9527_app_server_protocol::EnvironmentStatusParams;
use agent9527_app_server_protocol::EnvironmentStatusResponse;
use agent9527_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use agent9527_app_server_protocol::ExperimentalFeatureListParams;
use agent9527_app_server_protocol::ExperimentalFeatureListResponse;
use agent9527_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use agent9527_app_server_protocol::FeedbackUploadParams;
use agent9527_app_server_protocol::FeedbackUploadResponse;
use agent9527_app_server_protocol::GetAccountParams;
use agent9527_app_server_protocol::GetAccountRateLimitsResponse;
use agent9527_app_server_protocol::GetAccountResponse;
use agent9527_app_server_protocol::GetAccountTokenUsageResponse;
use agent9527_app_server_protocol::GetAuthStatusParams;
use agent9527_app_server_protocol::GetAuthStatusResponse;
use agent9527_app_server_protocol::GetConversationSummaryParams;
use agent9527_app_server_protocol::GetConversationSummaryResponse;
use agent9527_app_server_protocol::GetWorkspaceMessagesResponse;
use agent9527_app_server_protocol::GitDiffToRemoteParams;
use agent9527_app_server_protocol::GitDiffToRemoteResponse;
use agent9527_app_server_protocol::GitInfo as ApiGitInfo;
use agent9527_app_server_protocol::HookMetadata;
use agent9527_app_server_protocol::HooksListParams;
use agent9527_app_server_protocol::HooksListResponse;
use agent9527_app_server_protocol::InitializeParams;
use agent9527_app_server_protocol::InitializeResponse;
use agent9527_app_server_protocol::InstalledApp;
use agent9527_app_server_protocol::JSONRPCErrorError;
use agent9527_app_server_protocol::ListMcpServerStatusParams;
use agent9527_app_server_protocol::ListMcpServerStatusResponse;
use agent9527_app_server_protocol::LoginAccountParams;
use agent9527_app_server_protocol::LoginAccountResponse;
use agent9527_app_server_protocol::LoginApiKeyParams;
use agent9527_app_server_protocol::LoginAppBrand;
use agent9527_app_server_protocol::LogoutAccountResponse;
use agent9527_app_server_protocol::MarketplaceAddParams;
use agent9527_app_server_protocol::MarketplaceAddResponse;
use agent9527_app_server_protocol::MarketplaceInterface;
use agent9527_app_server_protocol::MarketplaceRemoveParams;
use agent9527_app_server_protocol::MarketplaceRemoveResponse;
use agent9527_app_server_protocol::MarketplaceUpgradeErrorInfo;
use agent9527_app_server_protocol::MarketplaceUpgradeParams;
use agent9527_app_server_protocol::MarketplaceUpgradeResponse;
use agent9527_app_server_protocol::McpResourceReadParams;
use agent9527_app_server_protocol::McpResourceReadResponse;
use agent9527_app_server_protocol::McpServerOauthLoginCompletedNotification;
use agent9527_app_server_protocol::McpServerOauthLoginParams;
use agent9527_app_server_protocol::McpServerOauthLoginResponse;
use agent9527_app_server_protocol::McpServerRefreshResponse;
use agent9527_app_server_protocol::McpServerStatus;
use agent9527_app_server_protocol::McpServerStatusDetail;
use agent9527_app_server_protocol::McpServerToolCallParams;
use agent9527_app_server_protocol::McpServerToolCallResponse;
use agent9527_app_server_protocol::MemoryResetResponse;
use agent9527_app_server_protocol::MockExperimentalMethodParams;
use agent9527_app_server_protocol::MockExperimentalMethodResponse;
use agent9527_app_server_protocol::ModelListParams;
use agent9527_app_server_protocol::ModelListResponse;
use agent9527_app_server_protocol::PermissionProfileListParams;
use agent9527_app_server_protocol::PermissionProfileListResponse;
use agent9527_app_server_protocol::PermissionProfileSummary;
use agent9527_app_server_protocol::PluginDetail;
use agent9527_app_server_protocol::PluginInstallParams;
use agent9527_app_server_protocol::PluginInstallResponse;
use agent9527_app_server_protocol::PluginInstalledParams;
use agent9527_app_server_protocol::PluginInstalledResponse;
use agent9527_app_server_protocol::PluginInterface;
use agent9527_app_server_protocol::PluginListMarketplaceKind;
use agent9527_app_server_protocol::PluginListParams;
use agent9527_app_server_protocol::PluginListResponse;
use agent9527_app_server_protocol::PluginMarketplaceEntry;
use agent9527_app_server_protocol::PluginReadParams;
use agent9527_app_server_protocol::PluginReadResponse;
use agent9527_app_server_protocol::PluginShareCheckoutParams;
use agent9527_app_server_protocol::PluginShareCheckoutResponse;
use agent9527_app_server_protocol::PluginShareContext;
use agent9527_app_server_protocol::PluginShareDeleteParams;
use agent9527_app_server_protocol::PluginShareDeleteResponse;
use agent9527_app_server_protocol::PluginShareDiscoverability;
use agent9527_app_server_protocol::PluginShareListItem;
use agent9527_app_server_protocol::PluginShareListParams;
use agent9527_app_server_protocol::PluginShareListResponse;
use agent9527_app_server_protocol::PluginSharePrincipal;
use agent9527_app_server_protocol::PluginSharePrincipalType;
use agent9527_app_server_protocol::PluginShareSaveParams;
use agent9527_app_server_protocol::PluginShareSaveResponse;
use agent9527_app_server_protocol::PluginShareTarget;
use agent9527_app_server_protocol::PluginShareUpdateDiscoverability;
use agent9527_app_server_protocol::PluginShareUpdateTargetsParams;
use agent9527_app_server_protocol::PluginShareUpdateTargetsResponse;
use agent9527_app_server_protocol::PluginSkillReadParams;
use agent9527_app_server_protocol::PluginSkillReadResponse;
use agent9527_app_server_protocol::PluginSource;
use agent9527_app_server_protocol::PluginSummary;
use agent9527_app_server_protocol::PluginUninstallParams;
use agent9527_app_server_protocol::PluginUninstallResponse;
use agent9527_app_server_protocol::RateLimitResetCredit;
use agent9527_app_server_protocol::RateLimitResetCreditStatus;
use agent9527_app_server_protocol::RateLimitResetCreditsSummary;
use agent9527_app_server_protocol::RateLimitResetType;
use agent9527_app_server_protocol::RequestId;
use agent9527_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use agent9527_app_server_protocol::ReviewStartParams;
use agent9527_app_server_protocol::ReviewStartResponse;
use agent9527_app_server_protocol::ReviewTarget as ApiReviewTarget;
use agent9527_app_server_protocol::SandboxMode;
use agent9527_app_server_protocol::SendAddCreditsNudgeEmailParams;
use agent9527_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use agent9527_app_server_protocol::ServerNotification;
use agent9527_app_server_protocol::ServerRequestResolvedNotification;
use agent9527_app_server_protocol::SkillSummary;
use agent9527_app_server_protocol::SkillsConfigWriteParams;
use agent9527_app_server_protocol::SkillsConfigWriteResponse;
use agent9527_app_server_protocol::SkillsExtraRootsSetParams;
use agent9527_app_server_protocol::SkillsExtraRootsSetResponse;
use agent9527_app_server_protocol::SkillsListParams;
use agent9527_app_server_protocol::SkillsListResponse;
use agent9527_app_server_protocol::SortDirection;
use agent9527_app_server_protocol::Thread;
use agent9527_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use agent9527_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use agent9527_app_server_protocol::ThreadArchiveParams;
use agent9527_app_server_protocol::ThreadArchiveResponse;
use agent9527_app_server_protocol::ThreadArchivedNotification;
use agent9527_app_server_protocol::ThreadBackgroundTerminal;
use agent9527_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use agent9527_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use agent9527_app_server_protocol::ThreadBackgroundTerminalsListParams;
use agent9527_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use agent9527_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use agent9527_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use agent9527_app_server_protocol::ThreadClosedNotification;
use agent9527_app_server_protocol::ThreadCompactStartParams;
use agent9527_app_server_protocol::ThreadCompactStartResponse;
use agent9527_app_server_protocol::ThreadDecrementElicitationParams;
use agent9527_app_server_protocol::ThreadDecrementElicitationResponse;
use agent9527_app_server_protocol::ThreadDeleteParams;
use agent9527_app_server_protocol::ThreadDeleteResponse;
use agent9527_app_server_protocol::ThreadDeletedNotification;
use agent9527_app_server_protocol::ThreadForkParams;
use agent9527_app_server_protocol::ThreadForkResponse;
use agent9527_app_server_protocol::ThreadGoal;
use agent9527_app_server_protocol::ThreadGoalClearParams;
use agent9527_app_server_protocol::ThreadGoalClearResponse;
use agent9527_app_server_protocol::ThreadGoalClearedNotification;
use agent9527_app_server_protocol::ThreadGoalGetParams;
use agent9527_app_server_protocol::ThreadGoalGetResponse;
use agent9527_app_server_protocol::ThreadGoalSetParams;
use agent9527_app_server_protocol::ThreadGoalSetResponse;
use agent9527_app_server_protocol::ThreadGoalStatus;
use agent9527_app_server_protocol::ThreadGoalUpdatedNotification;
use agent9527_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use agent9527_app_server_protocol::ThreadHistoryMode;
use agent9527_app_server_protocol::ThreadIncrementElicitationParams;
use agent9527_app_server_protocol::ThreadIncrementElicitationResponse;
use agent9527_app_server_protocol::ThreadInjectItemsParams;
use agent9527_app_server_protocol::ThreadInjectItemsResponse;
use agent9527_app_server_protocol::ThreadItem;
use agent9527_app_server_protocol::ThreadItemEntry;
use agent9527_app_server_protocol::ThreadItemsListParams;
use agent9527_app_server_protocol::ThreadItemsListResponse;
use agent9527_app_server_protocol::ThreadListCwdFilter;
use agent9527_app_server_protocol::ThreadListParams;
use agent9527_app_server_protocol::ThreadListResponse;
use agent9527_app_server_protocol::ThreadLoadedListParams;
use agent9527_app_server_protocol::ThreadLoadedListResponse;
use agent9527_app_server_protocol::ThreadMemoryModeSetParams;
use agent9527_app_server_protocol::ThreadMemoryModeSetResponse;
use agent9527_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use agent9527_app_server_protocol::ThreadMetadataUpdateParams;
use agent9527_app_server_protocol::ThreadMetadataUpdateResponse;
use agent9527_app_server_protocol::ThreadNameUpdatedNotification;
use agent9527_app_server_protocol::ThreadReadParams;
use agent9527_app_server_protocol::ThreadReadResponse;
use agent9527_app_server_protocol::ThreadRealtimeAppendAudioParams;
use agent9527_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use agent9527_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use agent9527_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use agent9527_app_server_protocol::ThreadRealtimeAppendTextParams;
use agent9527_app_server_protocol::ThreadRealtimeAppendTextResponse;
use agent9527_app_server_protocol::ThreadRealtimeListVoicesResponse;
use agent9527_app_server_protocol::ThreadRealtimeStartParams;
use agent9527_app_server_protocol::ThreadRealtimeStartResponse;
use agent9527_app_server_protocol::ThreadRealtimeStartTransport;
use agent9527_app_server_protocol::ThreadRealtimeStopParams;
use agent9527_app_server_protocol::ThreadRealtimeStopResponse;
use agent9527_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use agent9527_app_server_protocol::ThreadResumeParams;
use agent9527_app_server_protocol::ThreadResumeResponse;
use agent9527_app_server_protocol::ThreadRollbackParams;
use agent9527_app_server_protocol::ThreadSearchOccurrence;
use agent9527_app_server_protocol::ThreadSearchOccurrencesParams;
use agent9527_app_server_protocol::ThreadSearchOccurrencesResponse;
use agent9527_app_server_protocol::ThreadSearchParams;
use agent9527_app_server_protocol::ThreadSearchResponse;
use agent9527_app_server_protocol::ThreadSearchResult;
use agent9527_app_server_protocol::ThreadSearchTextRange;
use agent9527_app_server_protocol::ThreadSetNameParams;
use agent9527_app_server_protocol::ThreadSetNameResponse;
use agent9527_app_server_protocol::ThreadSettings;
use agent9527_app_server_protocol::ThreadSettingsUpdateParams;
use agent9527_app_server_protocol::ThreadSettingsUpdateResponse;
use agent9527_app_server_protocol::ThreadShellCommandParams;
use agent9527_app_server_protocol::ThreadShellCommandResponse;
use agent9527_app_server_protocol::ThreadSortKey;
use agent9527_app_server_protocol::ThreadSourceKind;
use agent9527_app_server_protocol::ThreadStartParams;
use agent9527_app_server_protocol::ThreadStartResponse;
use agent9527_app_server_protocol::ThreadStartedNotification;
use agent9527_app_server_protocol::ThreadStatus;
use agent9527_app_server_protocol::ThreadTurnsListParams;
use agent9527_app_server_protocol::ThreadTurnsListResponse;
use agent9527_app_server_protocol::ThreadUnarchiveParams;
use agent9527_app_server_protocol::ThreadUnarchiveResponse;
use agent9527_app_server_protocol::ThreadUnarchivedNotification;
use agent9527_app_server_protocol::ThreadUnsubscribeParams;
use agent9527_app_server_protocol::ThreadUnsubscribeResponse;
use agent9527_app_server_protocol::ThreadUnsubscribeStatus;
use agent9527_app_server_protocol::Turn;
use agent9527_app_server_protocol::TurnEnvironmentParams;
use agent9527_app_server_protocol::TurnError;
use agent9527_app_server_protocol::TurnInterruptParams;
use agent9527_app_server_protocol::TurnInterruptResponse;
use agent9527_app_server_protocol::TurnItemsView;
use agent9527_app_server_protocol::TurnStartParams;
use agent9527_app_server_protocol::TurnStartResponse;
use agent9527_app_server_protocol::TurnStatus;
use agent9527_app_server_protocol::TurnSteerParams;
use agent9527_app_server_protocol::TurnSteerResponse;
use agent9527_app_server_protocol::UserInput as V2UserInput;
use agent9527_app_server_protocol::WindowsSandboxReadiness;
use agent9527_app_server_protocol::WindowsSandboxReadinessResponse;
use agent9527_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use agent9527_app_server_protocol::WindowsSandboxSetupMode;
use agent9527_app_server_protocol::WindowsSandboxSetupStartParams;
use agent9527_app_server_protocol::WindowsSandboxSetupStartResponse;
use agent9527_app_server_protocol::WorkspaceMessage;
use agent9527_app_server_protocol::WorkspaceMessageType;
use agent9527_arg0::Arg0DispatchPaths;
use agent9527_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use agent9527_backend_client::Agent9527WorkspaceMessage as BackendWorkspaceMessage;
use agent9527_backend_client::Agent9527WorkspaceMessageType as BackendWorkspaceMessageType;
use agent9527_backend_client::Agent9527WorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use agent9527_backend_client::Client as BackendClient;
use agent9527_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use agent9527_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use agent9527_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use agent9527_backend_client::RequestError as BackendRequestError;
use agent9527_backend_client::TokenUsageProfile;
use agent9527_chatgpt::connectors;
use agent9527_chatgpt::workspace_settings;
use agent9527_config::CloudConfigBundleLoadError;
use agent9527_config::CloudConfigBundleLoadErrorCode;
use agent9527_config::ConfigLayerStack;
use agent9527_config::loader::project_trust_key;
use agent9527_config::types::McpServerTransportConfig;
use agent9527_connectors::AppInfo;
use agent9527_core::Agent9527Thread;
use agent9527_core::Agent9527ThreadSettingsOverrides;
use agent9527_core::ForkSnapshot;
use agent9527_core::McpManager;
use agent9527_core::NewThread;
#[cfg(test)]
use agent9527_core::SessionMeta;
use agent9527_core::StartThreadOptions;
use agent9527_core::SteerInputError;
use agent9527_core::ThreadConfigSnapshot;
use agent9527_core::ThreadManager;
use agent9527_core::config::Config;
use agent9527_core::config::ConfigOverrides;
use agent9527_core::config::NetworkProxyAuditMetadata;
use agent9527_core::config::edit::ConfigEdit;
use agent9527_core::config::edit::ConfigEditsBuilder;
use agent9527_core::connectors::AccessibleConnectorsStatus;
use agent9527_core::exec::ExecCapturePolicy;
use agent9527_core::exec::ExecExpiration;
use agent9527_core::exec::ExecParams;
use agent9527_core::exec_env::create_env;
use agent9527_core::path_utils;
#[cfg(test)]
use agent9527_core::read_head_for_summary;
use agent9527_core::sandboxing::SandboxPermissions;
use agent9527_core::truncate_rollout_after_turn_id;
use agent9527_core::truncate_rollout_before_turn_id;
use agent9527_core::windows_sandbox::WindowsSandboxLevelExt;
use agent9527_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use agent9527_core::windows_sandbox::WindowsSandboxSetupRequest;
use agent9527_core::windows_sandbox::sandbox_setup_is_complete;
use agent9527_core_plugins::PluginInstallError as CorePluginInstallError;
use agent9527_core_plugins::PluginInstallRequest;
use agent9527_core_plugins::PluginReadRequest;
use agent9527_core_plugins::PluginUninstallError as CorePluginUninstallError;
use agent9527_core_plugins::PluginsManager;
use agent9527_core_plugins::loader::load_plugin_apps;
use agent9527_core_plugins::loader::load_plugin_mcp_servers;
use agent9527_core_plugins::manifest::PluginManifestInterface;
use agent9527_core_plugins::marketplace::MarketplaceError;
use agent9527_core_plugins::marketplace::MarketplacePluginSource;
use agent9527_core_plugins::marketplace_add::MarketplaceAddError;
use agent9527_core_plugins::marketplace_add::MarketplaceAddRequest;
use agent9527_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_agent9527_home;
use agent9527_core_plugins::marketplace_remove::MarketplaceRemoveError;
use agent9527_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use agent9527_core_plugins::marketplace_remove::remove_marketplace;
use agent9527_core_plugins::remote::RemoteMarketplace;
use agent9527_core_plugins::remote::RemoteMarketplaceSource;
use agent9527_core_plugins::remote::RemotePluginCatalogError;
use agent9527_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use agent9527_core_plugins::remote::RemotePluginServiceConfig;
use agent9527_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use agent9527_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use agent9527_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use agent9527_exec_server::EnvironmentManager;
use agent9527_exec_server::EnvironmentObservedStatus;
use agent9527_exec_server::LOCAL_ENVIRONMENT_ID;
use agent9527_exec_server::LOCAL_FS;
use agent9527_features::FEATURES;
use agent9527_features::Feature;
use agent9527_features::Stage;
use agent9527_feedback::Agent9527Feedback;
use agent9527_feedback::FeedbackAttachmentPath;
use agent9527_feedback::FeedbackUploadOptions;
use agent9527_git_utils::git_diff_to_remote;
use agent9527_git_utils::resolve_root_git_project_for_trust;
use agent9527_login::AGENT9527_OPEN_APP_URL;
use agent9527_login::Agent9527Auth;
use agent9527_login::AuthManager;
use agent9527_login::LoginSuccessPage;
use agent9527_login::LoginSuccessPageBrand;
use agent9527_login::ServerOptions as LoginServerOptions;
use agent9527_login::ShutdownHandle;
use agent9527_login::complete_device_code_login;
use agent9527_login::login_with_api_key;
use agent9527_login::login_with_bedrock_api_key;
use agent9527_login::oauth_client_id;
use agent9527_login::request_device_code;
use agent9527_login::run_login_server;
use agent9527_mcp::McpRuntimeContext;
use agent9527_mcp::McpServerStatusSnapshot;
use agent9527_mcp::McpSnapshotDetail;
use agent9527_mcp::collect_mcp_server_status_snapshot_with_detail;
use agent9527_mcp::discover_supported_scopes_with_http_client;
use agent9527_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use agent9527_mcp::resolve_oauth_scopes;
use agent9527_memories_write::clear_memory_roots_contents;
use agent9527_model_provider::create_model_provider;
use agent9527_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use agent9527_protocol::ThreadId;
use agent9527_protocol::config_types::CollaborationMode;
use agent9527_protocol::config_types::ForcedLoginMethod;
use agent9527_protocol::config_types::Personality;
use agent9527_protocol::config_types::ReasoningSummary;
use agent9527_protocol::config_types::TrustLevel;
use agent9527_protocol::config_types::WindowsSandboxLevel;
use agent9527_protocol::error::Agent9527Err;
use agent9527_protocol::error::Result as Agent9527Result;
#[cfg(test)]
use agent9527_protocol::items::TurnItem;
use agent9527_protocol::models::ResponseItem;
use agent9527_protocol::openai_models::ReasoningEffort;
#[cfg(test)]
use agent9527_protocol::permissions::FileSystemSandboxPolicy;
use agent9527_protocol::protocol::AgentStatus;
use agent9527_protocol::protocol::ConversationAudioParams;
use agent9527_protocol::protocol::ConversationSpeechParams;
use agent9527_protocol::protocol::ConversationStartParams;
use agent9527_protocol::protocol::ConversationStartTransport;
use agent9527_protocol::protocol::ConversationTextParams;
use agent9527_protocol::protocol::EventMsg;
#[cfg(test)]
use agent9527_protocol::protocol::GitInfo as CoreGitInfo;
use agent9527_protocol::protocol::InitialHistory;
use agent9527_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use agent9527_protocol::protocol::Op;
use agent9527_protocol::protocol::RealtimeVoicesList;
use agent9527_protocol::protocol::ResumedHistory;
use agent9527_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use agent9527_protocol::protocol::ReviewRequest;
use agent9527_protocol::protocol::ReviewTarget as CoreReviewTarget;
use agent9527_protocol::protocol::RolloutItem;
use agent9527_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use agent9527_protocol::protocol::SessionMetaLine;
use agent9527_protocol::protocol::TurnEnvironmentSelection;
use agent9527_protocol::protocol::TurnEnvironmentSelections;
use agent9527_protocol::protocol::W3cTraceContext;
use agent9527_protocol::protocol::strip_user_message_prefix;
use agent9527_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use agent9527_protocol::user_input::UserInput as CoreInputItem;
use agent9527_rmcp_client::perform_oauth_login_return_url_with_http_client;
use agent9527_rollout::is_persisted_rollout_item;
use agent9527_rollout::state_db::StateDbHandle;
use agent9527_rollout::state_db::reconcile_rollout;
use agent9527_state::ThreadMetadata;
use agent9527_state::log_db::LogDbLayer;
use agent9527_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use agent9527_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use agent9527_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use agent9527_thread_store::GitInfoPatch as StoreGitInfoPatch;
use agent9527_thread_store::ItemSortKey as StoreItemSortKey;
use agent9527_thread_store::ListItemsParams as StoreListItemsParams;
use agent9527_thread_store::ListThreadsParams as StoreListThreadsParams;
use agent9527_thread_store::ListTurnsParams as StoreListTurnsParams;
use agent9527_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use agent9527_thread_store::LocalThreadStore;
use agent9527_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use agent9527_thread_store::ReadThreadParams as StoreReadThreadParams;
use agent9527_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use agent9527_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use agent9527_thread_store::SortDirection as StoreSortDirection;
use agent9527_thread_store::StoredThread;
use agent9527_thread_store::StoredTurn;
use agent9527_thread_store::StoredTurnItemsView;
use agent9527_thread_store::StoredTurnStatus;
use agent9527_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use agent9527_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use agent9527_thread_store::ThreadSortKey as StoreThreadSortKey;
use agent9527_thread_store::ThreadStore;
use agent9527_thread_store::ThreadStoreError;
use agent9527_utils_absolute_path::AbsolutePathBuf;
use agent9527_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use agent9527_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_processor;
mod plugins;
mod process_exec_processor;
mod remote_control_processor;
mod search;
mod thread_fork_goal;
mod thread_processor;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
            workspace_roots,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;
pub(crate) use self::thread_summary::thread_settings_from_core_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(
            item,
            agent9527_protocol::protocol::ThreadHistoryMode::Legacy,
        ) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
