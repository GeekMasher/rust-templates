use anyhow::Result;
use log::{debug, error};

mod cli;
mod config;

use crate::cli::*;

fn main() -> Result<()> {
    let arguments = init();
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
