use anyhow::Result;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub options: Options,
    pub settings: IndexMap<String, IndexMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Options {
    pub master_path: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            master_path: Some(r"~/.config/chsat".to_string()),
        }
    }
}

impl Config {
    pub fn parse(config: PathBuf) -> Result<Config> {
        let content = std::fs::read_to_string(config)?;
        let cfg = toml::from_str(&content)?;
        Ok(cfg)
    }
    pub fn parse_from_txt(context: &str) -> Result<Config> {
        let cfg = toml::from_str(&context)?;
        Ok(cfg)
    }
}
