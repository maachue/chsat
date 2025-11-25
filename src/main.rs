use anyhow::Result;
use clap::Parser;

use crate::cli::Commands;

mod cli;
mod config;
mod runner;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        Commands::Install { config } => {
            let cfg = config::Config::parse(config)?;
            println!("{:?}", cfg);

            runner::runner(
                &cfg.taskmanager.run,
                &cfg.tasks,
                &cfg.options.packagemanager,
            )?;
        }
        Commands::Set { settings } => {
            println!("{}", settings);
        }
    }
    Ok(())
}
