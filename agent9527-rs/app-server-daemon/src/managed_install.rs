use std::path::Path;
use std::path::PathBuf;

#[cfg(unix)]
use anyhow::Context;
#[cfg(unix)]
use anyhow::Result;
#[cfg(unix)]
use anyhow::anyhow;
#[cfg(unix)]
use sha2::Digest;
#[cfg(unix)]
use sha2::Sha256;
#[cfg(unix)]
use tokio::fs;
#[cfg(unix)]
use tokio::process::Command;

pub(crate) fn managed_agent9527_bin(agent9527_home: &Path) -> PathBuf {
    agent9527_home
        .join("packages")
        .join("standalone")
        .join("current")
        .join(managed_agent9527_file_name())
}

#[cfg(unix)]
pub(crate) async fn resolved_managed_agent9527_bin(agent9527_bin: &Path) -> Result<PathBuf> {
    fs::canonicalize(agent9527_bin).await.with_context(|| {
        format!(
            "failed to resolve managed Agent9527 binary {}",
            agent9527_bin.display()
        )
    })
}

#[cfg(unix)]
pub(crate) async fn managed_agent9527_version(agent9527_bin: &Path) -> Result<String> {
    let output = Command::new(agent9527_bin)
        .arg("--version")
        .output()
        .await
        .with_context(|| {
            format!(
                "failed to invoke managed Agent9527 binary {}",
                agent9527_bin.display()
            )
        })?;
    if !output.status.success() {
        return Err(anyhow!(
            "managed Agent9527 binary {} exited with status {}",
            agent9527_bin.display(),
            output.status
        ));
    }

    let stdout = String::from_utf8(output.stdout).with_context(|| {
        format!(
            "managed Agent9527 version was not utf-8: {}",
            agent9527_bin.display()
        )
    })?;
    parse_agent9527_version(&stdout)
}

#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutableIdentity {
    digest: [u8; 32],
}

#[cfg(unix)]
pub(crate) async fn executable_identity(executable: &Path) -> Result<ExecutableIdentity> {
    let bytes = fs::read(executable)
        .await
        .with_context(|| format!("failed to read executable {}", executable.display()))?;
    Ok(executable_identity_from_bytes(&bytes))
}

#[cfg(unix)]
pub(crate) fn executable_identity_from_bytes(bytes: &[u8]) -> ExecutableIdentity {
    ExecutableIdentity {
        digest: Sha256::digest(bytes).into(),
    }
}

fn managed_agent9527_file_name() -> &'static str {
    if cfg!(windows) {
        "agent9527.exe"
    } else {
        "agent9527"
    }
}

#[cfg(unix)]
fn parse_agent9527_version(output: &str) -> Result<String> {
    let version = output
        .split_whitespace()
        .nth(1)
        .filter(|version| !version.is_empty())
        .ok_or_else(|| anyhow!("managed Agent9527 version output was malformed"))?;
    Ok(version.to_string())
}

#[cfg(all(test, unix))]
#[path = "managed_install_tests.rs"]
mod tests;
