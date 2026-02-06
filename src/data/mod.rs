pub mod get;
mod ser;

#[cfg(windows)]
mod windows_util;

pub use ser::{
    ChsatData, ExposeData, KernelInfo, OSReleaseInfo, PMFlags, PackageManagerInfo, WindowsInfo,
};
