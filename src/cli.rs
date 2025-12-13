use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::utils;

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// TasksRunner based on TOML configuration
    Install {
        /// Task(s) to run
        #[arg(long, short, value_delimiter = ',', alias = "only", short_alias = 'o')]
        task: Option<Vec<String>>,

        /// Path to config file
        config: Option<PathBuf>,

        /// Parse from text
        #[arg(long = "from-txt", alias = "from-text")]
        config_txt: Option<String>,

        /// Show commands
        #[arg(long, alias = "validate")]
        dry_run: bool,

        /// Skip validation
        #[arg(long)]
        no_validate: bool,

        /// Debugging
        #[arg(long)]
        debug: bool,

        /// Skip confirmation
        #[arg(long)]
        no_confirm: bool,
    },
    /// Change settings as file based on TOML configuration
    Set {
        /// Setting key
        settings: Option<String>,
        /// New value for setting key
        value: Option<String>,

        /// Debugging
        #[arg(long)]
        debug: bool,

        /// Path to settings config
        #[arg(long, short, default_value = get_default_path().into_os_string())]
        config: Option<PathBuf>,

        /// Initialize settings file
        #[arg(long, short)]
        init: bool,

        /// Skip confirmation
        #[arg(long)]
        no_confirm: bool,

        /// DO not print
        #[arg(long)]
        no_display: bool,
    },
}

fn get_default_path() -> PathBuf {
    let path = "~/.config/yaat/settings.toml";
    let path = utils::resolve_path(path);
    path
}