pub(crate) mod cache;
pub mod collaboration_mode_presets;
pub(crate) mod config;
pub mod manager;
mod model_aliases;
pub mod model_info;
pub mod model_presets;
pub mod test_support;

pub use agent9527_protocol::auth::AuthMode;
pub use config::ModelsManagerConfig;
pub use model_aliases::MODEL_BALANCED;
pub use model_aliases::MODEL_FLASH;
pub use model_aliases::MODEL_PRO;
pub use model_aliases::canonical_model_id;

/// Load the bundled model catalog shipped with `agent9527-models-manager`.
pub fn bundled_models_response()
-> std::result::Result<agent9527_protocol::openai_models::ModelsResponse, serde_json::Error> {
    upstream_bundled_models_response().map(model_aliases::translate_models_response)
}

/// Load the unmodified upstream model catalog.
///
/// This is intended for provider adapters that need upstream metadata for provider-native IDs.
#[doc(hidden)]
pub fn upstream_bundled_models_response()
-> std::result::Result<agent9527_protocol::openai_models::ModelsResponse, serde_json::Error> {
    serde_json::from_str(include_str!("../models.json"))
}

/// Convert the client version string to a whole version string (e.g. "1.2.3-alpha.4" -> "1.2.3").
pub fn client_version_to_whole() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    )
}
