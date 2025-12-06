use anyhow::Result;
use owo_colors::OwoColorize;

use crate::settings::ERR;

#[derive(Debug, Clone)]
pub struct SettingsPath {
    pub master: String,
    pub sub: String,
}

pub fn resolver_key(input: &str) -> Result<SettingsPath> {
    let mut parts = input.split('.');

    let master = parts.next().unwrap().to_string();
    let sub = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("{} Expected format master.sub", ERR.red().bold()))?
        .to_string();

    Ok(SettingsPath { master, sub })
}

