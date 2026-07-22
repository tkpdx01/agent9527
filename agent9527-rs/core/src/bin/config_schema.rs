use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Generate the JSON Schema for `config.toml` and write it to `config.schema.json`.
#[derive(Parser)]
#[command(name = "agent9527-write-config-schema")]
struct Args {
    #[arg(short, long, value_name = "PATH")]
    out: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let out_path = args
        .out
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.schema.json"));
    agent9527_config::schema::write_config_schema(&out_path)?;
    Ok(())
}
