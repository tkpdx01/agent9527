use agent9527_api::ApiError;
use agent9527_protocol::error::Agent9527Err;
use http::StatusCode;

pub(super) const BEDROCK_EXPIRED_SIGNATURE_MESSAGE: &str = concat!(
    "Amazon Bedrock rejected the request because its AWS signature has expired. ",
    "Refresh your AWS credentials and retry. If `AWS_BEARER_TOKEN_BEDROCK` is set, ",
    "update or unset it, then restart Agent9527",
);

pub(super) fn map_api_error(error: ApiError) -> Agent9527Err {
    let mut error = agent9527_api::map_api_error(error);
    if let Agent9527Err::UnexpectedStatus(response) = &mut error
        && response.status == StatusCode::UNAUTHORIZED
        && response.body.contains("Signature expired:")
    {
        response.user_message = Some(BEDROCK_EXPIRED_SIGNATURE_MESSAGE.to_string());
    }
    error
}
