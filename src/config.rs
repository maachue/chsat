use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Config {
    pub options: Options,
    pub taskmanager: TaskManager,
    pub tasks: HashMap<String, Tasks>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct TaskManager {
    pub run: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Tasks {
    #[serde(rename = "install")]
    Install {
        #[serde(default)]
        flags: Vec<String>,
        #[serde(default)]
        pkgs: Vec<String>,
    },

    #[serde(rename = "remove")]
    Remove {
        #[serde(default)]
        flags: Vec<String>,
        #[serde(default)]
        pkgs: Vec<String>,
    },

    #[serde(rename = "update")]
    Update {
        #[serde(default)]
        flags: Vec<String>,
    },

    #[serde(rename = "shell")]
    Shell {
        #[serde(default)]
        program: String,
        #[serde(default)]
        flags: Vec<String>,
        #[serde(default)]
        args: Vec<String>,
    },
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Options {
    pub packagemanager: String,
}

impl Config {
    pub fn parse(config: PathBuf) -> Result<Config> {
        let context = std::fs::read_to_string(config)?;
        let cfg = toml::from_str(&context)?;
        Ok(cfg)
    }
}
