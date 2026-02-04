use color_eyre::Result;
use std::path::PathBuf;

pub fn get_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "maachue", "chsat").map(|c| PathBuf::from(c.config_dir()))
}

pub fn ask(msg: &str) -> Result<bool> {
    use dialoguer::Confirm;
    let confirmation = Confirm::new().with_prompt(msg).interact()?;

    Ok(confirmation)
}

pub fn read_stdin() -> Result<Option<String>> {
    use std::io::{self, IsTerminal, Read};

    if !io::stdin().is_terminal() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(Some(buf))
    } else {
        Ok(None)
    }
}

