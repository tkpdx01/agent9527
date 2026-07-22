use agent9527_protocol::openai_models::ModelInfo;
use agent9527_protocol::openai_models::ModelsResponse;
use std::borrow::Cow;
use std::collections::HashMap;

pub const MODEL_PRO: &str = "deepseek-v4-pro";
pub const MODEL_BALANCED: &str = "glm-5.2";
pub const MODEL_FLASH: &str = "deepseek-v4-flash";

/// Convert upstream bare GPT model identifiers into Agent9527's public model family.
///
/// Provider-qualified identifiers are intentionally left unchanged because they can be
/// provider-native routing keys, such as Amazon Bedrock's `openai.gpt-*` identifiers.
pub fn canonical_model_id(model: &str) -> Cow<'_, str> {
    let Some(gpt_suffix) = model.strip_prefix("gpt-") else {
        return Cow::Borrowed(model);
    };
    let lower = gpt_suffix.to_ascii_lowercase();
    let target = if lower.contains("luna") || lower.contains("mini") || lower.contains("nano") {
        MODEL_FLASH
    } else if lower.contains("terra") || lower.starts_with("5.4") {
        MODEL_BALANCED
    } else {
        MODEL_PRO
    };
    Cow::Borrowed(target)
}

pub(crate) fn translate_models_response(mut response: ModelsResponse) -> ModelsResponse {
    response.models = translate_models(response.models);
    response
}

pub(crate) fn translate_models(models: Vec<ModelInfo>) -> Vec<ModelInfo> {
    let mut translated = Vec::with_capacity(models.len());
    let mut indices = HashMap::<String, (usize, u8)>::new();

    for model in models {
        let source_slug = model.slug.clone();
        let target_slug = canonical_model_id(&source_slug).into_owned();
        let rank = source_rank(&source_slug, &target_slug);
        let model = translate_model(model, target_slug.clone());

        if let Some((index, current_rank)) = indices.get_mut(&target_slug) {
            if rank > *current_rank {
                translated[*index] = model;
                *current_rank = rank;
            }
        } else {
            indices.insert(target_slug, (translated.len(), rank));
            translated.push(model);
        }
    }

    translated
}

fn source_rank(source: &str, target: &str) -> u8 {
    if source == target {
        3
    } else if matches!(source, "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna") {
        2
    } else {
        1
    }
}

fn translate_model(mut model: ModelInfo, target_slug: String) -> ModelInfo {
    if model.slug == target_slug {
        return model;
    }
    model.slug = target_slug.clone();
    model.display_name = display_name(&target_slug)
        .map(str::to_string)
        .unwrap_or_else(|| translate_model_copy(&model.display_name));
    model.description = model.description.map(|copy| translate_model_copy(&copy));
    model.base_instructions = translate_model_copy(&model.base_instructions);

    if let Some(availability_nux) = model.availability_nux.as_mut() {
        availability_nux.message = translate_model_copy(&availability_nux.message);
    }
    if let Some(upgrade) = model.upgrade.as_mut() {
        upgrade.model = canonical_model_id(&upgrade.model).into_owned();
        upgrade.migration_markdown = translate_model_copy(&upgrade.migration_markdown);
    }
    if model
        .upgrade
        .as_ref()
        .is_some_and(|upgrade| upgrade.model == target_slug)
    {
        model.upgrade = None;
    }
    model.auto_review_model_override = model
        .auto_review_model_override
        .map(|model| canonical_model_id(&model).into_owned());

    if let Some(messages) = model.model_messages.as_mut() {
        translate_optional_copy(&mut messages.instructions_template);
        if let Some(variables) = messages.instructions_variables.as_mut() {
            translate_optional_copy(&mut variables.personality_default);
            translate_optional_copy(&mut variables.personality_friendly);
            translate_optional_copy(&mut variables.personality_pragmatic);
        }
        if let Some(approvals) = messages.approvals.as_mut() {
            translate_optional_copy(&mut approvals.on_request);
            translate_optional_copy(&mut approvals.on_request_auto_review);
        }
        if let Some(auto_review) = messages.auto_review.as_mut() {
            translate_optional_copy(&mut auto_review.policy);
            translate_optional_copy(&mut auto_review.policy_template);
        }
        if let Some(permissions) = messages.permissions.as_mut() {
            translate_optional_copy(&mut permissions.danger_full_access);
            translate_optional_copy(&mut permissions.workspace_write);
            translate_optional_copy(&mut permissions.read_only);
        }
    }

    model
}

fn translate_optional_copy(copy: &mut Option<String>) {
    if let Some(copy) = copy.as_mut() {
        *copy = translate_model_copy(copy);
    }
}

fn display_name(model: &str) -> Option<&'static str> {
    match model {
        MODEL_PRO => Some("DeepSeek V4 Pro"),
        MODEL_BALANCED => Some("GLM-5.2"),
        MODEL_FLASH => Some("DeepSeek V4 Flash"),
        _ => None,
    }
}

fn translate_model_copy(copy: &str) -> String {
    [
        ("GPT-5.6-Sol", "DeepSeek V4 Pro"),
        ("GPT-5.6 Sol", "DeepSeek V4 Pro"),
        ("GPT-5.6-Terra", "GLM-5.2"),
        ("GPT-5.6 Terra", "GLM-5.2"),
        ("GPT-5.6-Luna", "DeepSeek V4 Flash"),
        ("GPT-5.6 Luna", "DeepSeek V4 Flash"),
        ("GPT-5.4-Mini", "DeepSeek V4 Flash"),
        ("GPT-5.4 Mini", "DeepSeek V4 Flash"),
        ("GPT-5.5", "DeepSeek V4 Pro"),
        ("GPT-5.4", "GLM-5.2"),
        ("GPT-5.2", "DeepSeek V4 Pro"),
        ("GPT-5", "the configured model"),
    ]
    .into_iter()
    .fold(copy.to_string(), |copy, (source, target)| {
        copy.replace(source, target)
    })
}

#[cfg(test)]
#[path = "model_aliases_tests.rs"]
mod tests;
