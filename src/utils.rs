use anyhow::Result;
use std::{path::PathBuf};

pub const ERR: &str = "[ERR]";
pub const DEBUG: &str = "[DEBUG]";
pub const INFO: &str = "[INFO]";

pub fn resolve_path(path_str: &str) -> PathBuf {
    PathBuf::from(
        shellexpand::full(path_str)
            .unwrap()
            .into_owned()
            .replace("/", std::path::MAIN_SEPARATOR_STR.to_string().as_str()),
    )
}

pub fn ask(msg: &str) -> Result<bool> {
    use dialoguer::Confirm;
    let confirmation = Confirm::new().with_prompt(msg).interact()?;

    Ok(confirmation)
}

pub fn read_stdin() -> Result<Option<String>> {
    use std::io::{self, Read, IsTerminal};

    if ! io::stdin().is_terminal() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(Some(buf))
    } else {
        Ok(None)
    }
}