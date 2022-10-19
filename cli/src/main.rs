use clap::{Parser, Subcommand};
use anyhow::Result;
use log::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[clap(long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(subcommand)]
    commands: ArgumentCommands,

}

#[derive(Subcommand, Debug)]
enum ArgumentCommands { }


fn main() -> Result<()> {
    let arguments = Arguments::parse();

    // Logging 
    let mut level = log::LevelFilter::Info;
    if arguments.debug > 0 {
        info!("Debugging enabled...");
        level = log::LevelFilter::Debug;
    }
    env_logger::builder()
        .parse_default_env()
        .filter_level(level)
        .init();

    // Subcommands 
    match &arguments.commands {
        _ => {
            error!("Unsupported sub command...");
            todo!("Lets write some code...");
        }
    }
    
    Ok(())
}
