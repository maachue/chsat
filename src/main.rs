use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;

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
        } => {
            println!("{:?}", task.red().bold());

            let mut cfg = runner::config::Config::parse(config)?;

            if no_confirm != cfg.options.no_confirm {
                cfg.options.no_confirm = no_confirm;
            }

            if debug {
                println!("{} The config:\n {:?}", "[DEBUG]".red().bold(), cfg);
            }

            if let Some(tasks) = &task {
                println!("[INFO] Run specify task(s):");

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
        } => {
            if debug {
                println!("{:?}", settings)
            }

            // let cfg = settings_config::Config::parse(config.unwrap())?;

            // let Someset: Vec<&str> = settings.split('.').collect();

            // if debug {
            //     println!("{} Config has: {:?}", "[DEBUG]".red().bold(), cfg);
            // }

            // settings::manage(&cfg, &set, &value.unwrap(), true, true)?;
        }
    }
    Ok(())
}
