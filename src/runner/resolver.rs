use indexmap::IndexMap;

use crate::runner::{BuiltCommand, config::Tasks};

pub fn resolve_tasks<'a>(
    order: &Option<Vec<String>>,
    tasks: &'a IndexMap<String, Tasks>,
) -> Vec<&'a Tasks> {
    order
        .as_deref()
        .map(|o| o.iter().filter_map(|k| tasks.get(k)).collect())
        .unwrap_or_else(|| tasks.values().collect())
}

pub trait ToCommand {
    fn to_cmd(&self, pm: &str) -> BuiltCommand;
}

impl ToCommand for Tasks {
    fn to_cmd(&self, pm: &str) -> BuiltCommand {
        match self {
            Tasks::Install {
                flags,
                pkgs,
                is_sudo,
            }
            | Tasks::Remove {
                flags,
                pkgs,
                is_sudo,
            } => BuiltCommand::new(
                pm,
                flags.iter().cloned().chain(pkgs.iter().cloned()).collect(),
                *is_sudo,
            ),

            Tasks::Update { flags, is_sudo } => BuiltCommand::new(pm, flags.clone(), *is_sudo),

            Tasks::Shell {
                program,
                flags,
                args,
                is_sudo,
            } => BuiltCommand::new(
                program,
                flags.iter().cloned().chain(args.iter().cloned()).collect(),
                *is_sudo,
            ),
        }
    }
}

