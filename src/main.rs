use clap::Parser;
use color_eyre::Result;

use crate::cli::Commands;
mod cli;
mod config_toml;
mod data;
mod rhai_impl;
// mod utils;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();

    // let stdin = utils::read_stdin()?;

    match cmd.command {
        Commands::Data => {}
        _ => todo!(),
    }

    Ok(())
}
