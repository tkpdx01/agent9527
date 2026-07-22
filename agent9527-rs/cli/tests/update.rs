use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn agent9527_command(agent9527_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(agent9527_utils_cargo_bin::cargo_bin("agent9527")?);
    cmd.env("AGENT9527_HOME", agent9527_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let agent9527_home = TempDir::new()?;

    agent9527_command(agent9527_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains(
            "`agent9527 update` is not available in debug builds",
        ));

    Ok(())
}
