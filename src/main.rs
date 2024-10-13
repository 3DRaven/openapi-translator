use anyhow::Result;
use log::info;
use openapi_translator::init_logger;
use openapi_translator::services::cli;

fn main() -> Result<()> {
    init_logger();
    info!("======== TRANSLATION START ========");
    cli::visit_commands()?;
    info!("======== TRANSLATION FINISHED ========");
    Ok(())
}
