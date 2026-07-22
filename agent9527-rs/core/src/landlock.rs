use crate::spawn::SpawnChildRequest;
use crate::spawn::StdioPolicy;
use crate::spawn::spawn_child_async;
use agent9527_network_proxy::NetworkProxy;
use agent9527_protocol::models::PermissionProfile;
use agent9527_sandboxing::landlock::AGENT9527_LINUX_SANDBOX_ARG0;
use agent9527_sandboxing::landlock::allow_network_for_proxy;
use agent9527_sandboxing::landlock::create_linux_sandbox_command_args_for_permission_profile;
use agent9527_utils_absolute_path::AbsolutePathBuf;
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Child;

/// Spawn a shell tool command under the Linux sandbox helper
/// (agent9527-linux-sandbox), which defaults to bubblewrap for filesystem
/// isolation plus seccomp for network restrictions.
///
/// Unlike macOS Seatbelt where we directly embed the policy text, the Linux
/// helper is a separate executable. We pass the canonical permission profile
/// as JSON and let the helper derive the runtime filesystem/network policies.
#[allow(clippy::too_many_arguments)]
pub async fn spawn_command_under_linux_sandbox<P>(
    agent9527_linux_sandbox_exe: P,
    command: Vec<String>,
    command_cwd: AbsolutePathBuf,
    permission_profile: &PermissionProfile,
    sandbox_policy_cwd: &AbsolutePathBuf,
    use_legacy_landlock: bool,
    stdio_policy: StdioPolicy,
    network: Option<&NetworkProxy>,
    env: HashMap<String, String>,
) -> std::io::Result<Child>
where
    P: AsRef<Path>,
{
    let network_sandbox_policy = permission_profile.network_sandbox_policy();
    let args = create_linux_sandbox_command_args_for_permission_profile(
        command,
        command_cwd.as_path(),
        permission_profile,
        sandbox_policy_cwd,
        use_legacy_landlock,
        allow_network_for_proxy(/*enforce_managed_network*/ false),
    );
    let agent9527_linux_sandbox_exe = agent9527_linux_sandbox_exe.as_ref();
    // Preserve the helper alias when we already have it; otherwise force argv0
    // so arg0 dispatch still reaches the Linux sandbox path.
    let arg0 = if agent9527_linux_sandbox_exe
        .file_name()
        .and_then(|name| name.to_str())
        == Some(AGENT9527_LINUX_SANDBOX_ARG0)
    {
        // Old bubblewrap builds without `--argv0` need a real helper path whose
        // basename still dispatches to the Linux sandbox entrypoint.
        agent9527_linux_sandbox_exe.to_string_lossy().into_owned()
    } else {
        AGENT9527_LINUX_SANDBOX_ARG0.to_string()
    };
    spawn_child_async(SpawnChildRequest {
        program: agent9527_linux_sandbox_exe.to_path_buf(),
        args,
        arg0: Some(&arg0),
        cwd: command_cwd,
        network_sandbox_policy,
        network,
        stdio_policy,
        env,
    })
    .await
}
