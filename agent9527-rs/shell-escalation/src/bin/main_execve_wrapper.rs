#[cfg(not(unix))]
fn main() {
    eprintln!("agent9527-execve-wrapper is only implemented for UNIX");
    std::process::exit(1);
}

#[cfg(unix)]
pub use agent9527_shell_escalation::main_execve_wrapper as main;
