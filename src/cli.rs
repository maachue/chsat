use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the expose data
    Data,
    /// Validate the configuration (include: is the program in path yet?, etc)
    Validate {
        /// Sets config file
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        config: Option<PathBuf>,
        /// Imports the string as config to run
        #[arg(long, conflicts_with = "config")]
        from_str: Option<String>,
    },
    Run {
        /// Sets config file
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        config: Option<PathBuf>,
        /// Print all command will execute
        #[arg(long)]
        dry_run: bool,
        /// Imports the string as config to run
        #[arg(long, conflicts_with = "config")]
        from_str: Option<String>,
        /// Don't ask confirmation when execute the program
        #[arg(long)]
        no_confirm: bool,
    },
}
