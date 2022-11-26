use anyhow::Result;
use clap::Parser;
use log::{info, debug, error, warn};

mod cli;

use crate::cli::{ArgumentCommands, Arguments};


fn main() -> Result<()> {
    let arguments = Arguments::parse();

    let log_level = match arguments.debug {
        false => log::LevelFilter::Info,
        true => log::LevelFilter::Debug
    };

    env_logger::builder()
        .parse_default_env()
        .filter_level(log_level);

    debug!("Finished initialising, starting main workflow...");

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
