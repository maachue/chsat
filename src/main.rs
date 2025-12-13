use std::io::IsTerminal;

use anyhow::{Result, bail};
use clap::Parser;
use owo_colors::OwoColorize;
use utils::{DEBUG, ERR, INFO};

use crate::cli::Commands;

mod cli;
mod runner;
mod settings;
mod utils;

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
            config_txt,
        } => {
            // PIPELINES LOGIC
            let stdin = utils::read_stdin()?;

            let mut cfg = if let Some(txt) = stdin {
                runner::config::Config::parse_from_txt(&txt)?
            } else if let Some(txt) = config_txt {
                runner::config::Config::parse_from_txt(&txt)?
            } else if let Some(path) = config {
                runner::config::Config::parse(path)?
            } else {
                bail!("{} Config not found", "[ERR]".red().bold());
            };

            // OVERRIDE NO CONFIRM
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
        Commands::Set {
            settings,
            value,
            debug,
            config,
            init,
            no_confirm,
            no_display,
            config_txt,
        } => {
            let display = !no_display; // this is my fault

            // PIPELINES LOGIC
            let stdin = utils::read_stdin()?;

            let cfg = if let Some(txt) = stdin {
                crate::settings::config::Config::parse_from_txt(&txt)?
            } else if let Some(txt) = config_txt {
                crate::settings::config::Config::parse_from_txt(&txt)?
            } else if let Some(path) = config {
                crate::settings::config::Config::parse(path)?
            } else {
                bail!("{} Config not found", "[ERR]".red().bold());
            };

            // DONT ASK INTERACTIVE WHEN PIPPING
            let no_confirm = no_confirm || !std::io::stdin().is_terminal();

            if debug {
                println!("{} {:?}", DEBUG.red().bold(), settings);
                println!("{} settings:\n{:?}", DEBUG.red().bold(), cfg);
            }

            if init {
                crate::settings::init::init(&cfg, display, no_confirm)?;
                return Ok(());
            }

            match (settings, value) {
                (Some(s), Some(v)) => {
                    settings::manage(&cfg, &s, &v, display, no_confirm)?;
                }
                (Some(_), None) => {
                    bail!("{} Missing value. `--help` to see usage", ERR.red().bold())
                }
                (None, Some(_)) => {
                    bail!(
                        "{} Missing setting. `--help` to see usage",
                        ERR.red().bold()
                    )
                }
                _ => {
                    bail!("{} Nothing to do.", ERR.red().bold())
                }
            }
        }
    }
    Ok(())
}
