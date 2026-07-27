const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum Agent9527AccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_agent9527_access_token(access_token: &str) -> Agent9527AccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        Agent9527AccessToken::PersonalAccessToken(access_token)
    } else {
        Agent9527AccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
