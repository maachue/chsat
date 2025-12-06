use std::fs;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{settings::{build::build_path, config::Config}, utils::{self, ERR, resolve_path}};

pub fn init(config: &Config, display: bool, no_confirm: bool) -> Result<()> {
    let master_path = config.options.master_path.clone()
                        .ok_or_else(|| anyhow::anyhow!("{} master_path is not defined", ERR.bold().red()))?;

    let master_path = resolve_path(&master_path);

    for (master, subs) in &config.settings {
        for (sub, value) in subs {
            let path = build_path(&master_path, &master, &sub);
            println!("Will create: {:?} = {}", path, value);
            if no_confirm || utils::ask("Are you sure?")? {
                continue;
            }

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            fs::write(&path, value)?;
        }
    }

    Ok(())
}