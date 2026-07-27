use agent9527_api::ApiError;
use agent9527_protocol::error::Agent9527Err;
use agent9527_protocol::error::Agent9527ErrorDetails;
use http::StatusCode;

pub(super) const BEDROCK_EXPIRED_SIGNATURE_MESSAGE: &str = concat!(
    "Amazon Bedrock rejected the request because its AWS signature has expired. ",
    "Refresh your AWS credentials and retry. If `AWS_BEARER_TOKEN_BEDROCK` is set, ",
    "update or unset it, then restart Agent9527",
);

pub(super) fn map_api_error(error: ApiError) -> Agent9527Err {
    let error = agent9527_api::map_api_error(error);
    if let Agent9527ErrorDetails::UnexpectedStatus(response) = error.details()
        && response.status == StatusCode::UNAUTHORIZED
        && response.body.contains("Signature expired:")
    {
        let mut response = response.clone();
        response.user_message = Some(BEDROCK_EXPIRED_SIGNATURE_MESSAGE.to_string());
        let mapped_error = Agent9527Err::new(Agent9527ErrorDetails::UnexpectedStatus(response));
        return match error.retry_delay() {
            Some(retry_delay) => mapped_error.with_retry_delay(retry_delay),
            None => mapped_error,
        };
    }
    error
}
