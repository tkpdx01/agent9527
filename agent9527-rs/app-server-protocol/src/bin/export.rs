use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    about = "Generate TypeScript bindings and JSON Schemas for the Agent9527 app-server protocol"
)]
struct Args {
    /// Output directory where generated files will be written
    #[arg(short = 'o', long = "out", value_name = "DIR")]
    out_dir: PathBuf,

    /// Optional Prettier executable path to format generated TypeScript files
    #[arg(short = 'p', long = "prettier", value_name = "PRETTIER_BIN")]
    prettier: Option<PathBuf>,

    /// Include experimental API methods and fields in generated output.
    #[arg(long = "experimental")]
    experimental: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    agent9527_app_server_protocol::generate_ts_with_options(
        &args.out_dir,
        args.prettier.as_deref(),
        agent9527_app_server_protocol::GenerateTsOptions {
            experimental_api: args.experimental,
            ..agent9527_app_server_protocol::GenerateTsOptions::default()
        },
    )?;
    agent9527_app_server_protocol::generate_json_with_experimental(&args.out_dir, args.experimental)
}
