use crate::{
    runner::{
        config::Config,
        frontend::FrontEnd,
        resolver::{ToCommand, resolve_tasks},
        validate::{is_define, validate},
    },
    utils,
};
use color_eyre::Result;
use owo_colors::OwoColorize;

pub mod config;
pub mod frontend;
pub mod resolver;
pub mod validate;

pub struct BuiltCommand {
    pub program: String,
    pub args: Vec<String>,
    pub is_sudo: bool,
}

impl BuiltCommand {
    pub fn new(program: &str, args: Vec<String>, is_sudo: bool) -> Self {
        Self {
            program: program.to_string(),
            args,
            is_sudo,
        }
    }
}

impl std::fmt::Display for BuiltCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_sudo {
            write!(f, "sudo {} {}", self.program, self.args.join(" "))
        } else {
            write!(f, "{} {}", self.program, self.args.join(" "))
        }
    }
}

pub fn manage(config: &Config, dry_run: bool, no_validate: bool, no_confirm: bool) -> Result<()> {
    is_define(&config.taskmanager.run, &config.tasks)?;

    let list = resolve_tasks(&config.taskmanager.run, &config.tasks);

    let cmds: Vec<BuiltCommand> = list
        .iter()
        .map(|t| (**t).to_cmd(&config.options.packagemanager))
        .collect();

    if dry_run {
        println!("{} The command(s) will exectute:", "[INFO]".cyan().bold());
    }

    for cmd in cmds {
        if dry_run {
            if !no_validate {
                validate(&cmd.program)?;
            }
            print!(" - ");
            cmd.dry_run();
        } else {
            if !no_validate {
                validate(&cmd.program)?;
            }

            if no_confirm {
                cmd.execute()?;
            } else {
                println!(
                    "{} The command will execute: {}",
                    "[INFO]".cyan().bold(),
                    cmd
                );

                if utils::ask("exectute this command?")? {
                    cmd.execute()?
                }
            }
        }
    }

    Ok(())
}
