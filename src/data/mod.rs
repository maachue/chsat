pub mod get;
mod ser;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

pub use ser::{
    ChsatData, ExposeData, KernelInfo, OSReleaseInfo, PMFlags, PackageManagerInfo, WindowsInfo,
};
