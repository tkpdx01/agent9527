//! Agent9527 distribution policy kept separate from upstream feature behavior.

use std::ffi::OsStr;
use std::sync::OnceLock;

static EXTERNAL_API_ONLY_ACTIVE: OnceLock<()> = OnceLock::new();

pub const EXTERNAL_API_ONLY_ENV: &str = "AGENT9527_EXTERNAL_API_ONLY";
pub const ACCOUNT_AUTHENTICATION_DISABLED_MESSAGE: &str =
    "Agent9527 does not support account login. Configure an external API provider instead.";
pub const EXTERNAL_PROVIDER_REQUIRED_MESSAGE: &str = "Agent9527 only supports external API providers. Configure `model_provider` with `requires_openai_auth = false`, or set AGENT9527_API_BASE_URL, AGENT9527_API_KEY, and AGENT9527_MODEL.";

pub fn account_authentication_enabled() -> bool {
    false
}

pub fn ensure_account_authentication_enabled() -> Result<(), &'static str> {
    account_authentication_enabled()
        .then_some(())
        .ok_or(ACCOUNT_AUTHENTICATION_DISABLED_MESSAGE)
}

pub fn activate_external_api_only() {
    let _ = EXTERNAL_API_ONLY_ACTIVE.set(());
}

pub fn external_api_only_enabled() -> bool {
    EXTERNAL_API_ONLY_ACTIVE.get().is_some()
        || std::env::var_os(EXTERNAL_API_ONLY_ENV)
            .as_deref()
            .is_some_and(env_flag_enabled)
}

pub fn validate_model_provider(provider_requires_openai_auth: bool) -> Result<(), &'static str> {
    if external_api_only_enabled() && provider_requires_openai_auth {
        Err(EXTERNAL_PROVIDER_REQUIRED_MESSAGE)
    } else {
        Ok(())
    }
}

fn env_flag_enabled(value: &OsStr) -> bool {
    let value = value.to_string_lossy();
    !value.is_empty()
        && !value.eq_ignore_ascii_case("0")
        && !value.eq_ignore_ascii_case("false")
        && !value.eq_ignore_ascii_case("off")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_authentication_is_disabled() {
        assert!(!account_authentication_enabled());
        assert_eq!(
            ensure_account_authentication_enabled(),
            Err(ACCOUNT_AUTHENTICATION_DISABLED_MESSAGE)
        );
    }

    #[test]
    fn native_distribution_activation_requires_external_providers() {
        activate_external_api_only();

        assert!(external_api_only_enabled());
        assert_eq!(
            validate_model_provider(true),
            Err(EXTERNAL_PROVIDER_REQUIRED_MESSAGE)
        );
        assert_eq!(validate_model_provider(false), Ok(()));
    }

    #[test]
    fn environment_flag_values_are_parsed_explicitly() {
        assert!(env_flag_enabled(OsStr::new("1")));
        assert!(env_flag_enabled(OsStr::new("true")));
        assert!(!env_flag_enabled(OsStr::new("")));
        assert!(!env_flag_enabled(OsStr::new("0")));
        assert!(!env_flag_enabled(OsStr::new("FALSE")));
        assert!(!env_flag_enabled(OsStr::new("off")));
    }
}
