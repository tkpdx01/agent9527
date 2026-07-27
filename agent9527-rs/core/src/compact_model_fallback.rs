use agent9527_analytics::CompactionImplementation;
use agent9527_analytics::CompactionReason;
use agent9527_otel::SessionTelemetry;
use agent9527_protocol::error::Agent9527Err;
use agent9527_protocol::error::Agent9527ErrorDetails;
use tracing::warn;

/// Retries failures that may be model-specific and succeed with a different model.
pub(crate) fn should_retry_with_current_model(error: &Agent9527Err) -> bool {
    matches!(
        error.details(),
        Agent9527ErrorDetails::InvalidRequest(_)
            | Agent9527ErrorDetails::UnexpectedStatus(_)
            | Agent9527ErrorDetails::ContextWindowExceeded
            | Agent9527ErrorDetails::UsageLimitReached(_)
            | Agent9527ErrorDetails::ServerOverloaded
            | Agent9527ErrorDetails::InternalServerError
            | Agent9527ErrorDetails::RetryLimit(_)
    )
}

pub(crate) fn record_model_fallback(
    session_telemetry: &SessionTelemetry,
    previous_model: &str,
    current_model: &str,
    reason: CompactionReason,
    implementation: CompactionImplementation,
    fallback_error: Option<&Agent9527Err>,
) {
    let reason_tag = match reason {
        CompactionReason::UserRequested => "user_requested",
        CompactionReason::ContextLimit => "context_limit",
        CompactionReason::ModelDownshift => "model_downshift",
        CompactionReason::CompHashChanged => "comp_hash_changed",
    };
    let implementation_tag = match implementation {
        CompactionImplementation::Responses => "responses",
        CompactionImplementation::ResponsesCompactionV2 => "responses_compaction_v2",
        CompactionImplementation::ResponsesCompact => "responses_compact",
    };
    let outcome = if fallback_error.is_none() {
        "succeeded"
    } else {
        "failed"
    };
    session_telemetry.counter(
        "agent9527.compaction.model_fallback",
        /*inc*/ 1,
        &[
            ("reason", reason_tag),
            ("implementation", implementation_tag),
            ("outcome", outcome),
        ],
    );
    warn!(
        previous_model,
        current_model,
        ?reason,
        ?implementation,
        outcome,
        ?fallback_error,
        "previous-model compaction failed; retried with current model"
    );
}
