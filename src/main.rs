use anyhow::Result;
use log::{error, info};
use openapi_translator::enums::common::Script;
use openapi_translator::init_logger;
use openapi_translator::services::cli;

fn main() -> Result<()> {
    init_logger()?;
    info!("================================================ TRANSLATION START ================================================");
    cli::visit_commands().inspect_err(|err| {
        if let Err(error) = Script::ErrorHandler.call_func(Some(&format!("{:?}", err))) {
            error!("errorHandler function not called from LUA vm: [{}]", error)
        }
    })?;
    info!("================================================ TRANSLATION FINISHED ================================================");
    Ok(())
}
