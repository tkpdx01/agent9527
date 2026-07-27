use super::*;
use agent9527_protocol::error::Agent9527ErrorDetails;

pub(super) fn environment_selection_error(err: Agent9527Err) -> JSONRPCErrorError {
    match err.details() {
        Agent9527ErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
