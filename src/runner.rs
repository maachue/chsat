use std::{collections::HashMap, process::Command};

use anyhow::{Result, bail};

use crate::config::Tasks;

pub fn runner(
    taskmanager: &Option<Vec<String>>,
    tasks: &HashMap<String, Tasks>,
    pm: &str,
) -> Result<()> {
    if let Some(task_list) = taskmanager {
        for task_name in task_list {
            if let Some(task) = tasks.get(task_name) {
                match task {
                    Tasks::Install { flags, pkgs } => {
                        if flags.is_empty() && !pkgs.is_empty() {
                            bail!("No flags??")
                        }
                        if !pkgs.is_empty() {
                            packagemanager(pm, flags, pkgs)?;
                        }
                    }
                    Tasks::Remove { flags, pkgs } => {
                        if flags.is_empty() && !pkgs.is_empty() {
                            bail!("No flags??")
                        }
                        if !pkgs.is_empty() {
                            packagemanager(pm, flags, pkgs)?;
                        }
                    }
                    Tasks::Update { flags } => {
                        if flags.is_empty() {
                            bail!("No flags??")
                        } else {
                            packagemanager(pm, flags, &[])?;
                        }
                    }
                    Tasks::Shell {
                        program,
                        flags,
                        args,
                    } => {
                        shellrun(program, flags, args)?;
                    }
                }
            } else {
                eprintln!("Task not found: {}", task_name);
            }
        }
    } else {
        for task in tasks.values() {
            match task {
                Tasks::Install { flags, pkgs } => {
                    if flags.is_empty() && !pkgs.is_empty() {
                        bail!("No flags??")
                    }
                    if !pkgs.is_empty() {
                        packagemanager(pm, flags, pkgs)?;
                    }
                }
                Tasks::Remove { flags, pkgs } => {
                    if flags.is_empty() && !pkgs.is_empty() {
                        bail!("No flags??")
                    }
                    if !pkgs.is_empty() {
                        packagemanager(pm, flags, pkgs)?;
                    }
                }
                Tasks::Update { flags } => {
                    if flags.is_empty() {
                        bail!("No flags??")
                    } else {
                        packagemanager(pm, flags, &[])?;
                    }
                }
                Tasks::Shell {
                    program,
                    flags,
                    args,
                } => {
                    shellrun(program, flags, args)?;
                }
            }
        }
    }

    Ok(())
}

fn packagemanager(mg: &str, flags: &[String], pkgs: &[String]) -> Result<()> {
    let mut cmd = Command::new("sudo");
    cmd.arg(mg);
    cmd.args(flags);
    cmd.args(pkgs);

    // let status = cmd.status()?;
    // eprintln!("Exit status: {}", status);
    eprintln!("{:?}", cmd);

    Ok(())
}

fn shellrun(program: &str, flags: &[String], args: &[String]) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.args(flags);
    cmd.args(args);

    // let status = cmd.status()?;
    // eprintln!("Exit status: {}", status);

    eprintln!("{:?}", cmd);

    Ok(())
}
