use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set the log level
    #[arg(long, default_value = "INFO", global = true, value_parser = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"])]
    pub log_level: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Refresh the list of available projects
    Refresh,
    /// Print the project which currently used
    Current,
}

pub fn parse() -> Cli {
    Cli::parse()
}
