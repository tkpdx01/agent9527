use super::*;
use crate::upstream_bundled_models_response;
use agent9527_protocol::openai_models::ReasoningEffort;
use pretty_assertions::assert_eq;

#[test]
fn canonicalizes_bare_gpt_models_and_preserves_provider_ids() {
    assert_eq!(canonical_model_id("gpt-5.6-sol"), MODEL_PRO);
    assert_eq!(canonical_model_id("gpt-5.6-luna"), MODEL_FLASH);
    assert_eq!(canonical_model_id("gpt-5.6-terra"), MODEL_BALANCED);
    assert_eq!(canonical_model_id("gpt-5.7"), MODEL_PRO);
    assert_eq!(canonical_model_id("gpt-5.7-mini"), MODEL_FLASH);
    assert_eq!(
        canonical_model_id("openai.gpt-5.6-sol"),
        "openai.gpt-5.6-sol"
    );
    assert_eq!(
        canonical_model_id("provider/gpt-5.6-sol"),
        "provider/gpt-5.6-sol"
    );
}

#[test]
fn bundled_catalog_uses_public_models_and_preserves_reasoning_levels() {
    let upstream = upstream_bundled_models_response().expect("upstream catalog should parse");
    let translated = translate_models_response(upstream.clone());

    assert_eq!(
        translated
            .models
            .iter()
            .map(|model| model.slug.as_str())
            .collect::<Vec<_>>(),
        vec![
            MODEL_PRO,
            MODEL_BALANCED,
            MODEL_FLASH,
            "agent9527-auto-review"
        ]
    );

    for (source, target) in [
        ("gpt-5.6-sol", MODEL_PRO),
        ("gpt-5.6-terra", MODEL_BALANCED),
        ("gpt-5.6-luna", MODEL_FLASH),
    ] {
        let source = upstream
            .models
            .iter()
            .find(|model| model.slug == source)
            .expect("source model should exist");
        let target = translated
            .models
            .iter()
            .find(|model| model.slug == target)
            .expect("translated model should exist");
        assert_eq!(
            (
                target.default_reasoning_level.clone(),
                &target.supported_reasoning_levels,
                target.context_window,
                target.max_context_window,
            ),
            (
                source.default_reasoning_level.clone(),
                &source.supported_reasoning_levels,
                source.context_window,
                source.max_context_window,
            )
        );
    }

    let pro = translated
        .models
        .iter()
        .find(|model| model.slug == MODEL_PRO)
        .expect("pro model should exist");
    assert_eq!(pro.default_reasoning_level, Some(ReasoningEffort::Low));
    assert!(!pro.base_instructions.contains("GPT-5"));
}

#[test]
fn native_target_metadata_wins_over_an_upstream_alias() {
    let mut models = upstream_bundled_models_response()
        .expect("upstream catalog should parse")
        .models;
    let mut native = models
        .iter()
        .find(|model| model.slug == "gpt-5.5")
        .expect("source model should exist")
        .clone();
    native.slug = MODEL_PRO.to_string();
    native.display_name = "Provider Native Pro".to_string();
    native.context_window = Some(999_000);
    models.push(native);

    let translated = translate_models(models);
    let pro = translated
        .iter()
        .find(|model| model.slug == MODEL_PRO)
        .expect("pro model should exist");

    assert_eq!(pro.context_window, Some(999_000));
    assert_eq!(pro.display_name, "Provider Native Pro");
}
