use std::path::PathBuf;
use anyhow::Result;

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
    let confirmation = Confirm::new()
        .with_prompt(msg)
        .interact()?;

    Ok(confirmation)
}