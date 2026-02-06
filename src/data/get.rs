use std::path::Path;

#[cfg(windows)]
use crate::data::windows_util;
use crate::data::{KernelInfo, OSReleaseInfo, PackageManagerInfo, WindowsInfo};

use super::ExposeData;
use color_eyre::{Result, eyre::eyre};

impl KernelInfo {
    pub fn get() -> Result<Self> {
        #[cfg(target_family = "unix")]
        {
            Ok(Self {
                osrelease: sys_info::os_release().unwrap_or("".to_string()),
                ostype: sys_info::os_type().unwrap_or("".to_string()),
                version: String::from_utf8(Command::new("uname").arg("-v").output()?.stdout)
                    .unwrap_or_default()
                    .trim() // Remove `\n`
                    .to_string(),
            })
        }

        #[cfg(not(target_family = "unix"))]
        Ok(Self::default())
    }
}

impl OSReleaseInfo {
    pub fn get() -> Result<Self> {
        #[cfg(target_family = "unix")]
        {
            let os = sys_info::linux_os_release()?;
            Ok(Self {
                bug_report_url: os.bug_report_url.unwrap_or_default(),
                build_id: os.build_id.unwrap_or_default(),
                documentation_url: os.documentation_url.unwrap_or_default(),
                home_url: os.home_url.unwrap_or_default(),
                id: os.id.unwrap_or_default(),
                logo: os.logo.unwrap_or_default(),
                name: os.name.unwrap_or_default(),
                pretty_name: os.pretty_name.unwrap_or_default(),
                privacy_policy_url: os.privacy_policy_url.unwrap_or_default(),
                support_url: os.support_url.unwrap_or_default(),
            })
        }
        #[cfg(not(target_family = "unix"))]
        Ok(Self::default())
    }
}

impl PackageManagerInfo {
    pub fn get(os_id: &str) -> Self {
        match os_id {
            "cachyos" | "archlinux" | "endeavouros" => Self {
                binary_name: "pacman".to_string(),
                binary_path: which::which("pacman").unwrap_or_default(),
                flags: super::PMFlags {
                    install_flags: vec!["-S".to_string()],
                    remove_flags: vec!["-Rns".to_string()],
                    update_pkgs_flags: vec!["-Syu".to_string()],
                    update_metadata_flags: vec!["-Syy".to_string()],
                },
                usage: "{{pm}} {{flags}} {{pkgs}} {{opts}}".to_string(),
                required_sudo: true,
            },
            _ => Self::default(),
        }
    }
}

impl WindowsInfo {
    pub fn get() -> Result<Self> {
        if cfg!(windows) {
            windows_util::get_windowsnt_infomation().map_err(|e| eyre!(e))
        } else {
            Ok(Self::default())
        }
    }
}

impl ExposeData {
    pub fn new(args: &[String], config: &Path) -> Result<Self> {
        let os_release = OSReleaseInfo::get()?;

        let os_id = if std::env::consts::OS == "windows" {
            std::env::consts::OS
        } else {
            &os_release.id
        };

        Ok(Self {
            chsat: super::ChsatData {
                arch: std::env::consts::ARCH,
                args: args.to_owned(),
            },
            command_dir: std::fs::canonicalize(Path::new(".")).unwrap_or_default(),
            config_file: config.to_path_buf(),
            executable: which::which("chsat").unwrap_or_default(),
            home_dir: std::env::home_dir().unwrap_or_default(),
            hostname: sys_info::hostname().unwrap_or_default(),
            kernel: KernelInfo::get()?,
            os: std::env::consts::OS,
            package_manager: PackageManagerInfo::get(os_id),
            os_release,
            // WARN: `users` crate support only unix
            // uid: users::get_current_uid().to_string(),
            // username: users::get_current_username()
            //     .unwrap_or_default()
            //     .to_string_lossy()
            //     .into_owned(),
            // gid: users::get_current_gid().to_string(),
            // group: users::get_current_groupname()
            //     .unwrap_or_default()
            //     .to_string_lossy()
            //     .into_owned(),
            windows_version: WindowsInfo::get()?,
        })
    }
}
