use anyhow::Result;
use log::{debug, error};

mod cli;

use crate::cli::*;

fn main() -> Result<()> {
    let arguments = init();
    debug!("Finished initialising, starting main workflow...");

    // Load Configuration
    let config = Config::load(arguments.config)?;

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
