use anyhow::{Result, bail};
use clap::Parser;
use owo_colors::OwoColorize;
use utils::{DEBUG, ERR, INFO};

use crate::cli::Commands;

mod cli;
mod runner;
// mod settings;
mod utils;
mod settings;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        Commands::Install {
            task,
            config,
            dry_run,
            no_validate,
            debug,
            no_confirm,
        } => {
            let mut cfg = runner::config::Config::parse(config)?;

            if no_confirm != cfg.options.no_confirm {
                cfg.options.no_confirm = no_confirm;
            }

            if debug {
                println!("{} The config:\n {:?}", DEBUG.red().bold(), cfg);
            }

            if let Some(tasks) = &task {
                println!("{} Run specify task(s):", INFO.blue().bold());

                for t in tasks {
                    println!(" - {}", t);
                }

                cfg.taskmanager.run = Some(tasks.clone());
            }

            runner::manage(&cfg, dry_run, no_validate, cfg.options.no_confirm)?;
        }
        Commands::Set { settings,
            value,
            debug,
            config,
            init } => {
            let cfg = crate::settings::config::Config::parse(config.unwrap())?;
            if debug {
                println!("{} {:?}", DEBUG.red().bold(), settings);
                println!("{} settings:\n{:?}", DEBUG.red().bold(), cfg);
            }

            if init {
                crate::settings::init::init(&cfg, true, false)?;
                return Ok(())
            }

            match (settings, value) {
                (Some(s), Some(v)) => {
                    settings::manage(&cfg, &s, &v, true, true)?;
                }
                (Some(_), None) => {
                    bail!("{} Missing value. `--help` to see usage", ERR.red().bold())
                }
                (None, Some(_)) => {
                    bail!("{} Missing setting. `--help` to see usage", ERR.red().bold())
                }
                _ => {
                    bail!("{} Nothing to do.", ERR.red().bold())
                }
            }
        }
    }
    Ok(())
}
