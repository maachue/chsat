use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Task(s) to run
    #[arg(long, short, value_delimiter = ',', alias = "only", short_alias = 'o')]
    pub task: Option<Vec<String>>,

    /// Path to config file
    #[arg(value_parser = clap::value_parser!(PathBuf))]
    pub config: Option<PathBuf>,

    /// Parse from text
    #[arg(long = "from-str", alias = "from-text", conflicts_with = "config")]
    pub config_txt: Option<String>,

    /// Show commands
    #[arg(long, alias = "validate")]
    pub dry_run: bool,

    /// Skip validation
    #[arg(long)]
    pub no_validate: bool,

    /// Debugging
    #[arg(long)]
    pub debug: bool,

    /// Skip confirmation
    #[arg(long)]
    pub no_confirm: bool,
}
