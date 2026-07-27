use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

use agent9527_utils_home_dir::find_agent9527_home;
use fluent_bundle::FluentArgs;
use fluent_bundle::FluentResource;
use fluent_bundle::concurrent::FluentBundle;
use unic_langid::LanguageIdentifier;

const UI_LANGUAGE_FILE: &str = "ui-language";
const UI_LANGUAGE_ENV: &str = "AGENT9527_UI_LANGUAGE";
const SYSTEM_LOCALE_ENV: &str = "AGENT9527_SYSTEM_LOCALE";

mod language_pack;

pub(crate) use language_pack::discover_language_packs;
use language_pack::is_english_locale;
pub(crate) use language_pack::language_pack_root;
use language_pack::resolve_language_pack;

#[cfg(test)]
#[path = "i18n_tests.rs"]
mod tests;

/// Resolves stable message IDs from an external Fluent bundle and falls back
/// to the caller-provided English text for missing or malformed messages.
pub(crate) struct Localizer {
    locale: Option<LanguageIdentifier>,
    bundle: Option<FluentBundle<FluentResource>>,
}

impl Localizer {
    pub(crate) fn english() -> Self {
        Self {
            locale: None,
            bundle: None,
        }
    }

    pub(crate) fn from_ftl(locale: &str, source: &str) -> Self {
        let Ok(locale) = locale.parse::<LanguageIdentifier>() else {
            return Self::english();
        };
        let Ok(resource) = FluentResource::try_new(source.to_string()) else {
            return Self::english();
        };
        let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
        bundle.set_use_isolating(false);
        if bundle.add_resource(resource).is_err() {
            return Self::english();
        }
        Self {
            locale: Some(locale),
            bundle: Some(bundle),
        }
    }

    pub(crate) fn from_runtime() -> Self {
        let Ok(agent9527_home) = find_agent9527_home() else {
            return Self::english();
        };
        let requested_locale = explicit_locale()
            .or_else(|| read_language_preference(&agent9527_home))
            .or_else(system_locale)
            .unwrap_or_else(|| "en".to_string());
        let root = language_pack_root(&agent9527_home);
        Self::from_language_pack_root(&requested_locale, &root)
    }

    pub(crate) fn from_language_pack_root(requested_locale: &str, root: &Path) -> Self {
        if is_english_locale(requested_locale) {
            return Self::english();
        }
        let Ok(candidates) = discover_language_packs(root) else {
            return Self::english();
        };
        let Some(candidate) = resolve_language_pack(requested_locale, &candidates) else {
            return Self::english();
        };
        let Some(source) = candidate.source.as_deref() else {
            return Self::english();
        };
        Self::from_ftl(&candidate.locale, source)
    }

    pub(crate) fn text<F>(&self, key: &str, args: Option<&FluentArgs>, english: F) -> String
    where
        F: FnOnce() -> String,
    {
        let Some(bundle) = self.bundle.as_ref() else {
            return english();
        };
        let Some(message) = bundle.get_message(key) else {
            return english();
        };
        let Some(pattern) = message.value() else {
            return english();
        };
        let mut errors = Vec::new();
        let value = bundle.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() || value.trim().is_empty() {
            return english();
        }
        value.into_owned()
    }

    pub(crate) fn text_with_string_arg<F>(
        &self,
        key: &str,
        arg_name: &str,
        arg_value: impl Into<String>,
        english: F,
    ) -> String
    where
        F: FnOnce() -> String,
    {
        let mut args = FluentArgs::new();
        args.set(arg_name, arg_value.into());
        self.text(key, Some(&args), english)
    }
}

impl Default for Localizer {
    fn default() -> Self {
        Self::english()
    }
}

fn explicit_locale() -> Option<String> {
    non_empty_env(UI_LANGUAGE_ENV)
}

fn system_locale() -> Option<String> {
    [SYSTEM_LOCALE_ENV, "LC_ALL", "LC_MESSAGES", "LANG"]
        .into_iter()
        .find_map(non_empty_env)
        .map(|locale| {
            locale
                .split('.')
                .next()
                .unwrap_or(&locale)
                .replace('_', "-")
        })
}

fn non_empty_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn language_preference_path(agent9527_home: &Path) -> PathBuf {
    agent9527_home.join(UI_LANGUAGE_FILE)
}

fn read_language_preference(agent9527_home: &Path) -> Option<String> {
    fs::read_to_string(language_preference_path(agent9527_home))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub(crate) fn active_locale() -> String {
    global()
        .locale
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "en".to_string())
}

pub(crate) fn save_language_preference(
    agent9527_home: &Path,
    input: &str,
) -> Result<String, String> {
    let localizer = global();
    let root = language_pack_root(agent9527_home);
    let candidates = discover_language_packs(&root)?;
    let locale = if is_english_locale(input) {
        "en".to_string()
    } else if let Some(candidate) = resolve_language_pack(input, &candidates) {
        candidate.locale.clone()
    } else {
        return Err(localizer.text_with_string_arg(
            "language-unsupported",
            "locale",
            input.trim(),
            || {
                format!(
                    "Language {} is not installed or compatible. Use /language to see available options.",
                    input.trim()
                )
            },
        ));
    };
    fs::create_dir_all(agent9527_home)
        .map_err(|error| format!("Could not create AGENT9527_HOME: {error}"))?;
    fs::write(
        language_preference_path(agent9527_home),
        format!("{locale}\n"),
    )
    .map_err(|error| format!("Could not save language preference: {error}"))?;
    Ok(
        localizer.text_with_string_arg("language-saved", "locale", &locale, || {
            format!("Selected {locale}; restart Agent9527 to apply.")
        }),
    )
}

pub(crate) fn global() -> &'static Localizer {
    static LOCALIZER: OnceLock<Localizer> = OnceLock::new();
    LOCALIZER.get_or_init(Localizer::from_runtime)
}
