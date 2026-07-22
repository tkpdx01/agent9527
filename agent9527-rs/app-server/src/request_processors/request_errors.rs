use super::*;

pub(super) fn environment_selection_error(err: Agent9527Err) -> JSONRPCErrorError {
    match err {
        Agent9527Err::InvalidRequest(message) => invalid_request(message),
        err => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
