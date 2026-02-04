use color_eyre::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub options: Options,
    #[serde(default)]
    pub taskmanager: TaskManager,
    pub tasks: IndexMap<String, Tasks>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct TaskManager {
    pub run: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Tasks {
    #[serde(rename = "install")]
    Install {
        flags: Vec<String>,
        pkgs: Vec<String>,
        is_sudo: bool,
    },

    #[serde(rename = "remove")]
    Remove {
        flags: Vec<String>,
        pkgs: Vec<String>,
        is_sudo: bool,
    },

    #[serde(rename = "update")]
    Update { flags: Vec<String>, is_sudo: bool },

    #[serde(rename = "shell")]
    Shell {
        #[serde(default)]
        program: String,
        #[serde(default)]
        flags: Vec<String>,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        is_sudo: bool,
    },
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct Options {
    pub no_confirm: bool,
    pub packagemanager: String,
}

impl Config {
    pub fn parse(config: PathBuf) -> Result<Config> {
        let context = std::fs::read_to_string(config)?;
        let cfg = toml::from_str(&context)?;
        Ok(cfg)
    }

    pub fn parse_from_txt(context: &str) -> Result<Config> {
        let cfg = toml::from_str(context)?;
        Ok(cfg)
    }
}
