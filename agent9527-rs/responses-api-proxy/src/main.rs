use agent9527_responses_api_proxy::Args as ResponsesApiProxyArgs;
use clap::Parser;

#[ctor::ctor]
fn pre_main() {
    agent9527_process_hardening::pre_main_hardening();
}

pub fn main() -> anyhow::Result<()> {
    let args = ResponsesApiProxyArgs::parse();
    agent9527_responses_api_proxy::run_main(args)
}
