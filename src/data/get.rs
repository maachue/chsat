use std::{path::Path, process::Command};

use crate::data::{KernelInfo, OSReleaseInfo};

use super::ExposeData;
use color_eyre::{Result, eyre::OptionExt};

impl KernelInfo {
    pub fn get() -> Result<Self> {
        #[cfg(target_family = "unix")]
        {
            Ok(Self {
                osrelease: sys_info::os_release().unwrap_or("".to_string()),
                ostype: sys_info::os_type().unwrap_or("".to_string()),
                version: String::from_utf8(Command::new("uname").arg("-s").output()?.stdout)
                    .unwrap_or("".to_string()),
            })
        }

        #[cfg(target_family = "windows")]
        Ok(Self::default())
    }
}

impl OSReleaseInfo {
    pub fn get() -> Result<Self> {
        #[cfg(target_family = "unix")]
        {
            let sys = sys_info::linux_os_release()?;
            Ok(Self {
                bug_report_url: sys.bug_report_url.unwrap_or("".to_string()),
                build_id: sys.build_id.unwrap_or("".to_string()),
                documentation_url: sys.documentation_url.unwrap_or("".to_string()),
                home_url: sys.home_url.unwrap_or("".to_string()),
                id: sys.id.unwrap_or("".to_string()),
                logo: sys.logo.unwrap_or("".to_string()),
                name: sys.name.unwrap_or("".to_string()),
                pretty_name: sys.pretty_name.unwrap_or("".to_string()),
                privacy_policy_url: sys.privacy_policy_url.unwrap_or("".to_string()),
                support_url: sys.support_url.unwrap_or("".to_string()),
            })
        }
        #[cfg(target_family = "windows")]
        Ok(Self::default())
    }
}

impl ExposeData {
    pub fn new(args: Vec<String>, config: &Path) -> Result<Self> {
        Ok(Self {
            chsat: super::ChsatData {
                arch: "amd64",
                args,
            },
            command_dir: shellexpand::tilde(".").into_owned().into(),
            config_file: config.to_path_buf(),
            executable: which::which("chsat").unwrap_or("".into()),
            home_dir: std::env::home_dir().unwrap_or("".into()),
            hostname: sys_info::hostname().unwrap_or("".to_string()),
            kernel: KernelInfo::get()?,
            os: std::env::consts::OS,
            os_release: OSReleaseInfo::get()?,
            uid: users::get_current_uid().to_string(),
            username: users::get_current_username()
                .unwrap_or("".into())
                .to_string_lossy()
                .into_owned(),
            gid: users::get_current_gid().to_string(),
            group: users::get_current_groupname()
                .unwrap_or("".into())
                .to_string_lossy()
                .into_owned(),
            windows_version: super::WindowsInfo::default(),
            package_manager: super::PackageManagerInfo::default(),
        })
    }
}
