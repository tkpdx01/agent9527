use std::fs;
use std::path::Path;

use fluent_bundle::FluentArgs;
use pretty_assertions::assert_eq;
use sha2::Digest;
use sha2::Sha256;
use tempfile::TempDir;

use super::Localizer;
use super::discover_language_packs;
use super::language_pack_root;
use super::save_language_preference;

const TEST_FTL: &str = r#"
status-line-configure-title = 配置状态栏
history-worked-for = 工作了 { $duration }
language-saved = 已选择 { $locale }；重启 Codex 后生效。
language-unsupported = 语言 { $locale } 未安装或不兼容。
"#;

#[test]
fn static_message_uses_fluent_translation() {
    let localizer = Localizer::from_ftl("zh-CN", TEST_FTL);

    assert_eq!(
        localizer.text("status-line-configure-title", None, || {
            "Configure Status Line".to_string()
        }),
        "配置状态栏"
    );
}

#[test]
fn fluent_arguments_are_formatted() {
    let localizer = Localizer::from_ftl("zh-CN", TEST_FTL);
    let mut args = FluentArgs::new();
    args.set("duration", "7m 57s");

    assert_eq!(
        localizer.text("history-worked-for", Some(&args), || {
            "Worked for 7m 57s".to_string()
        }),
        "工作了 7m 57s"
    );
}

#[test]
fn missing_or_malformed_messages_use_english_fallback() {
    let localizer = Localizer::from_ftl("zh-CN", TEST_FTL);
    assert_eq!(
        localizer.text("missing-key", None, || "English fallback".to_string()),
        "English fallback"
    );

    let malformed = Localizer::from_ftl("zh-CN", "valid = 有效\nbroken = {");
    assert_eq!(
        malformed.text("valid", None, || "English fallback".to_string()),
        "English fallback"
    );
}

#[test]
fn external_language_pack_is_discovered_and_loaded() {
    let temp = TempDir::new().expect("temp dir");
    write_language_pack(
        temp.path(),
        "zh-CN",
        "zh-CN",
        /*api_min*/ 1,
        /*api_max*/ 1,
        TEST_FTL,
        /*hash_override*/ None,
    );

    let candidates = discover_language_packs(temp.path()).expect("discover packs");
    assert_eq!(candidates.len(), 1);
    assert!(candidates[0].is_available());
    assert_eq!(candidates[0].locale, "zh-CN");
    assert_eq!(candidates[0].display_name, "简体中文 (zh-CN)");

    let localizer = Localizer::from_language_pack_root("zh-Hans", temp.path());
    assert_eq!(
        localizer.text("status-line-configure-title", None, || {
            "Configure Status Line".to_string()
        }),
        "配置状态栏"
    );
}

#[test]
fn invalid_language_packs_are_disabled_without_panicking() {
    let temp = TempDir::new().expect("temp dir");
    write_language_pack(
        temp.path(),
        "fr-FR",
        "fr-FR",
        /*api_min*/ 2,
        /*api_max*/ 3,
        "probe = Bonjour",
        /*hash_override*/ None,
    );
    let invalid_hash = "0".repeat(64);
    write_language_pack(
        temp.path(),
        "zh-CN",
        "zh-CN",
        /*api_min*/ 1,
        /*api_max*/ 1,
        TEST_FTL,
        Some(&invalid_hash),
    );

    let candidates = discover_language_packs(temp.path()).expect("discover packs");
    assert_eq!(candidates.len(), 2);
    assert!(candidates.iter().all(|candidate| !candidate.is_available()));
    assert!(
        candidates
            .iter()
            .any(|candidate| candidate.disabled_reason.as_deref()
                == Some("Requires i18n API 2..=3; this Codex build provides 1."))
    );
    assert!(
        candidates
            .iter()
            .any(|candidate| candidate.disabled_reason.as_deref()
                == Some("messages.ftl SHA256 does not match manifest.json."))
    );
}

#[test]
fn missing_language_pack_root_is_an_empty_catalog() {
    let temp = TempDir::new().expect("temp dir");
    let candidates = discover_language_packs(&temp.path().join("missing")).expect("discover packs");
    assert!(candidates.is_empty());
}

#[test]
fn language_preference_accepts_an_installed_primary_language_alias() {
    let temp = TempDir::new().expect("temp dir");
    let root = language_pack_root(temp.path());
    write_language_pack(
        &root, "zh-CN", "zh-CN", /*api_min*/ 1, /*api_max*/ 1, TEST_FTL,
        /*hash_override*/ None,
    );

    save_language_preference(temp.path(), "zh").expect("save language preference");
    assert_eq!(
        fs::read_to_string(temp.path().join("ui-language")).expect("read preference"),
        "zh-CN\n"
    );
}

fn write_language_pack(
    root: &Path,
    directory: &str,
    locale: &str,
    api_min: u32,
    api_max: u32,
    source: &str,
    hash_override: Option<&str>,
) {
    let pack_dir = root.join(directory);
    fs::create_dir_all(&pack_dir).expect("create pack directory");
    fs::write(pack_dir.join("messages.ftl"), source).expect("write FTL");
    let hash = hash_override
        .map(str::to_string)
        .unwrap_or_else(|| format!("{:x}", Sha256::digest(source.as_bytes())));
    let manifest = serde_json::json!({
        "schemaVersion": 1,
        "type": "language",
        "id": format!("test.{locale}"),
        "locale": locale,
        "displayName": "Simplified Chinese",
        "nativeName": "简体中文",
        "i18nApi": {
            "min": api_min,
            "max": api_max
        },
        "resources": [
            {
                "path": "messages.ftl",
                "sha256": format!("sha256:{hash}")
            }
        ]
    });
    fs::write(
        pack_dir.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).expect("serialize manifest"),
    )
    .expect("write manifest");
}
