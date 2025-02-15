#![forbid(unsafe_code)]

use anyhow::Result;
use log::{debug, error};

mod cli;
mod config;

use crate::{cli::*, config::Config};

fn main() -> Result<()> {
    let arguments = init();
    debug!("Finished initialising, starting main workflow...");

    // Load Configuration
    let config = match Config::load(arguments.config) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    // [optional] config.arguments(&arguments);
    log::debug!("Configuration loaded: {:#?}", config);

    // Subcommands
    match &arguments.commands {
        // TODO: Add the different sub commands here
        _ => {
            error!("Unsupported sub command...");
            todo!("Lets write some code...");
        }
    }

    Ok(())
}
