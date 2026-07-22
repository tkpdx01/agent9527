#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    win::main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("agent9527-command-runner is Windows-only");
}
