//! Minimal exec-server fixture for Bazel-only integration tests.
//!
//! Linking only exec-server avoids depending on the full Agent9527 CLI binary
//! when a test only needs a WebSocket executor endpoint. It handles the arg0
//! helper mode because sandboxed process requests re-exec this binary.

use agent9527_exec_server::ExecServerRuntimePaths;
use agent9527_http_client::HttpClientFactory;
use agent9527_http_client::OutboundProxyPolicy;
#[cfg(unix)]
use std::ffi::OsStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(unix)]
    {
        let mut args = std::env::args_os();
        let _ = args.next();
        if args.next().as_deref()
            == Some(OsStr::new(
                agent9527_exec_server::AGENT9527_ARG0_EXEC_HELPER_ARG1,
            ))
        {
            agent9527_exec_server::run_arg0_exec_helper_main();
        }
    }

    let current_exe = std::env::current_exe()?;
    let runtime_paths =
        ExecServerRuntimePaths::new(current_exe, /*agent9527_linux_sandbox_exe*/ None)?;
    agent9527_exec_server::run_main(
        "ws://127.0.0.1:0",
        runtime_paths,
        HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault),
    )
    .await
}
