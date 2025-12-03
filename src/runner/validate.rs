use indexmap::IndexMap;
use owo_colors::OwoColorize;
use which::which;
use anyhow::{Context, Result, bail};

use crate::runner::config::Tasks;

pub fn validate(program: &str) -> Result<()> {
    which(&program)
        .with_context(|| format!("Cannot find binary path: {}", program))?;
    Ok(())
}

pub fn is_define<'a>(
    order: &Option<Vec<String>>,
    tasks: &'a IndexMap<String, Tasks>,
) -> Result<()> {
    if let Some(list) = order {
        for name in list {
            if !tasks.contains_key(name) {
                bail!("Task `{}` is not defined in [tasks]", name.green().bold())
            }
        }
    }

    Ok(())
}