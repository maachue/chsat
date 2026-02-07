use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

use crate::cli::Commands;
mod cli;
// mod config_toml;
mod data;
mod rhai_impl;
// mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cmd = cli::Cli::parse();

    // let stdin = utils::read_stdin()?;

    match cmd.command {
        Commands::Data => {
            let data = data::ExposeData::new(&["data".to_string()], &PathBuf::new())?;
            let json = serde_json::to_string_pretty(&data)?;
            println!("{}", json);
        }
        _ => todo!(),
    }

    Ok(())
}
