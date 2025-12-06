use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        #[arg(long, short, value_delimiter = ',', alias = "only", short_alias = 'o')]
        task: Option<Vec<String>>,

        /// path to your config
        config: PathBuf,

        /// print all command(s) will execute
        #[arg(long, alias = "validate")]
        dry_run: bool,

        /// non validate run (all will validate by default)
        #[arg(long)]
        no_validate: bool,

        /// debug (print all the config has)
        #[arg(long)]
        debug: bool,

        /// skip confirm
        #[arg(long)]
        no_confirm: bool,
    },
    Set {
        settings: Option<String>,
        value: Option<String>,

        /// debug
        #[arg(long)]
        debug: bool,

        /// config
        #[arg(long, short, default_value = "~/.config/dotctl/settings.toml")]
        config: Option<PathBuf>,

        // init the settings (from settings)
        #[arg(long, short)]
        init: bool,

        // skip confirm
        #[arg(long)]
        no_confirm: bool,

        #[arg(long)]
        no_display: bool,
    },
}
