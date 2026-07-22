//! Minimal exec-server fixture for Bazel-only integration tests.
//!
//! Linking only exec-server avoids depending on the full Agent9527 CLI binary
//! when a test only needs a WebSocket executor endpoint.

use agent9527_exec_server::ExecServerRuntimePaths;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let current_exe = std::env::current_exe()?;
    let runtime_paths =
        ExecServerRuntimePaths::new(current_exe, /*agent9527_linux_sandbox_exe*/ None)?;
    agent9527_exec_server::run_main("ws://127.0.0.1:0", runtime_paths).await
}
