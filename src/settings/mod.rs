mod build;
pub mod config;
mod exec;
pub mod init;
mod resolver;
mod validate;

use crate::settings::config::Config;
pub use crate::utils::ERR;
use anyhow::Result;

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

    let item = build::BuiltSettings::new(&config.options.master_path.clone().unwrap(), key, value);

    if display_flag {
        item.display();
    }

    if no_confirm || crate::utils::ask("Apply this change?")? {
        item.apply()?;
    }

    Ok(())
}
