use std::io;

use agent9527_extension_api::LoadUserInstructionsFuture;
use agent9527_extension_api::LoadedUserInstructions;
use agent9527_extension_api::UserInstructions;
use agent9527_extension_api::UserInstructionsProvider;
use agent9527_utils_absolute_path::AbsolutePathBuf;

const DEFAULT_AGENTS_MD_FILENAME: &str = "AGENTS.md";
const LOCAL_AGENTS_MD_FILENAME: &str = "AGENTS.override.md";

/// Loads user instructions from a Agent9527 home directory.
#[derive(Clone, Debug)]
pub struct Agent9527HomeUserInstructionsProvider {
    agent9527_home: AbsolutePathBuf,
}

impl Agent9527HomeUserInstructionsProvider {
    /// Creates a provider rooted at the supplied absolute Agent9527 home directory.
    pub fn new(agent9527_home: AbsolutePathBuf) -> Self {
        Self { agent9527_home }
    }

    async fn load_from_agent9527_home(&self) -> LoadedUserInstructions {
        let mut warnings = Vec::new();
        for candidate in [LOCAL_AGENTS_MD_FILENAME, DEFAULT_AGENTS_MD_FILENAME] {
            let path = self.agent9527_home.join(candidate);
            match tokio::fs::metadata(path.as_path()).await {
                Ok(metadata) if !metadata.is_file() => continue,
                Ok(_) => {}
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => {
                    warnings.push(format!(
                        "Failed to read global AGENTS.md instructions from `{}`: {err}",
                        path.display()
                    ));
                    continue;
                }
            }
            let data = match tokio::fs::read(path.as_path()).await {
                Ok(data) => data,
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => {
                    warnings.push(format!(
                        "Failed to read global AGENTS.md instructions from `{}`: {err}",
                        path.display()
                    ));
                    continue;
                }
            };
            let contents = String::from_utf8_lossy(&data);
            let trimmed = contents.trim();
            if !trimmed.is_empty() {
                return LoadedUserInstructions {
                    instructions: Some(UserInstructions {
                        text: trimmed.to_string(),
                        source: path,
                    }),
                    warnings,
                };
            }
        }
        LoadedUserInstructions {
            instructions: None,
            warnings,
        }
    }
}

impl UserInstructionsProvider for Agent9527HomeUserInstructionsProvider {
    fn load_user_instructions(&self) -> LoadUserInstructionsFuture<'_> {
        Box::pin(self.load_from_agent9527_home())
    }
}

#[cfg(test)]
mod tests;
