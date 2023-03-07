use std::io;

use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};

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
    pub project: Option<Vec<String>>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the project which currently used
    Current,

    /// List the available projects
    List,

    /// Refresh the list of available projects
    Refresh,

    /// Generate shell completions
    GenerateCompletions {
        #[arg(long, value_enum)]
        shell: Shell,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}

pub fn command() -> Command {
    Cli::command()
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
