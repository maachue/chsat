//! # Data
//!
//! chezmoi-like. Nothing more.
//!
use std::path::PathBuf;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExposeData {
    pub chsat: ChsatData,
    // pub command: String,
    pub command_dir: PathBuf,
    pub config_file: PathBuf,
    pub executable: PathBuf,
    pub home_dir: PathBuf,
    pub hostname: String,
    pub kernel: KernelInfo,
    pub os: &'static str,
    pub os_release: OSReleaseInfo,
    pub uid: String,
    pub username: String,
    pub gid: String,
    pub group: String,
    pub windows_version: WindowsInfo,
    pub package_manager: PackageManagerInfo,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChsatData {
    pub arch: &'static str,
    pub args: Vec<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KernelInfo {
    pub osrelease: String,
    pub ostype: String,
    pub version: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OSReleaseInfo {
    pub bug_report_url: String,
    pub build_id: String,
    pub documentation_url: String,
    pub home_url: String,
    pub id: String,
    pub logo: String,
    pub name: String,
    pub pretty_name: String,
    pub privacy_policy_url: String,
    pub support_url: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowsInfo {}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageManagerInfo {
    pub binary_name: String,
    pub binary_path: PathBuf,
    pub flags: PMFlags,
    pub usage: String,
    pub required_sudo: bool,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PMFlags {
    pub install_flags: Vec<String>,
    pub remove_flags: Vec<String>,
    pub update_pkgs_flags: Vec<String>,
    pub update_metadata_flags: Vec<String>,
}
