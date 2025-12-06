use std::path::{Path, PathBuf};

use crate::settings::resolver::SettingsPath;

use crate::utils::resolve_path;

pub struct BuiltSettings {
    pub path: PathBuf,
    pub master: String,
    pub sub: String,
    pub value: String,
}

pub fn build_path(base: &Path, master: &str, sub: &str) -> PathBuf {
    let mut p = base.to_path_buf();

    if master == "master" {
        p.push(sub);
    } else {
        p.push(master);
        p.push(sub);
    }

    p
}

impl BuiltSettings {
    pub fn new(base: &str, key: SettingsPath, value: &str) -> Self {
        let path = resolve_path(base);
        let path = build_path(&path, &key.master, &key.sub);
        Self {
            path,
            master: key.master,
            sub: key.sub,
            value: value.to_string(),
        }
    }
}
