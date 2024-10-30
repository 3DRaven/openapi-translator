use anyhow::Result;
use log::info;
use openapi_translator::services::cli;
use openapi_translator::{check_scripts, init_logger};

fn main() -> Result<()> {
    init_logger();
    check_scripts()?;
    info!("================================================ TRANSLATION START ================================================");
    cli::visit_commands()?;
    info!("================================================ TRANSLATION FINISHED ================================================");
    Ok(())
}
