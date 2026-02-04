use std::io::IsTerminal;

use clap::Parser;
use color_eyre::{Result, eyre::bail};
mod cli;
mod runner;
mod utils;

fn main() -> Result<()> {
    let mut cmd = cli::Cli::parse();

    let stdin = utils::read_stdin()?;

    if cmd.config_txt.is_none()
        && let Some(stdin) = stdin
    {
        cmd.config_txt = Some(stdin)
    };

    let mut cfg = if let Some(config) = cmd.config {
        runner::config::Config::parse(config)?
    } else if let Some(context) = cmd.config_txt {
        runner::config::Config::parse_from_txt(&context)?
    } else {
        bail!("[CONFIG] field is required.")
    };

    if cmd.no_confirm != cfg.options.no_confirm {
        cfg.options.no_confirm = cmd.no_confirm;
    }

    cfg.options.no_confirm = cmd.no_confirm || !std::io::stdin().is_terminal();

    if let Some(tasks) = &cmd.task {
        println!("Run specify task(s):");

        for t in tasks {
            println!(" - {}", t);
        }

        cfg.taskmanager.run = Some(tasks.clone());
    }

    runner::manage(&cfg, cmd.dry_run, cmd.no_validate, cfg.options.no_confirm)?;
    Ok(())
}
