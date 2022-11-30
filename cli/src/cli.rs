use clap::{Parser, Subcommand};


pub const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const BANNER: &str = r#" !! MY RUST CLI BANNER !!"#;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(long, help = "Enable Debugging", default_value_t = false)]
    pub debug: bool,

    #[clap(long, help = "Disable Banner", default_value_t = false)]
    pub disable_banner: bool,

    #[clap(
        short,
        long,
        help = "Configuration file path",
        default_value_t=String::from("./config.toml")
    )]
    pub config: String,

    #[clap(subcommand)]
    pub commands: ArgumentCommands,
}

#[derive(Subcommand, Debug)]
pub enum ArgumentCommands {
    // TODO: Subcommands
}

