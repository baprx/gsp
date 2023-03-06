use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set the log level
    #[arg(long, default_value = "INFO", global = true, value_parser = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"])]
    pub log_level: String,

    /// Force a cache refresh then run the requested command
    #[arg(long, default_value_t = false, global = true)]
    pub refresh: bool,

    /// Part or entire project ID
    pub project: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the project which currently used
    Current,
    /// List the available projects
    List,
    /// Refresh the list of available projects
    Refresh,
}

pub fn parse() -> Cli {
    Cli::parse()
}
