mod resolver;
mod validate;
mod build;
mod exec;
pub mod init;
pub mod config;

use anyhow::Result;
use crate::settings::config::Config;
pub use crate::utils::{ERR, DEBUG, INFO};

pub fn manage(
    config: &Config,
    raw_key: &str,
    value: &str,
    display_flag: bool,
    no_confirm: bool,
) -> Result<()> {
    use exec::Exec;
    let key = resolver::resolver_key(raw_key)?;
    validate::validate(config, &key)?;

    let item = build::BuiltSettings::new(
        &config.options.master_path.clone().unwrap(),
        key,
        value,
    );

    if display_flag {
        item.display();
    }

    if no_confirm || crate::utils::ask("Apply this change?")? {
        item.apply()?;
    }

    Ok(())
}
