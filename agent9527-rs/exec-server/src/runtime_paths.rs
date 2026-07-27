use std::path::PathBuf;

use agent9527_utils_absolute_path::AbsolutePathBuf;

/// Runtime paths needed by exec-server child processes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the Agent9527 executable used to launch hidden helper modes.
    pub agent9527_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter Agent9527 by argv0.
    pub agent9527_linux_sandbox_exe: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        agent9527_self_exe: Option<PathBuf>,
        agent9527_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let agent9527_self_exe = agent9527_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Agent9527 executable path is not configured",
            )
        })?;
        Self::new(agent9527_self_exe, agent9527_linux_sandbox_exe)
    }

    pub fn new(
        agent9527_self_exe: PathBuf,
        agent9527_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            agent9527_self_exe: absolute_path(agent9527_self_exe)?,
            agent9527_linux_sandbox_exe: agent9527_linux_sandbox_exe
                .map(absolute_path)
                .transpose()?,
        })
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
