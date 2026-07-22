use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_agent9527_access_token("at-example"),
        Agent9527AccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_agent9527_access_token("header.payload.signature"),
        Agent9527AccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
