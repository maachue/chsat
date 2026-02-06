use thiserror::Error;
use windows::{
    Win32::System::Registry::{
        HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_DWORD, REG_SZ, REG_VALUE_TYPE, RegCloseKey,
        RegOpenKeyExW, RegQueryValueExW,
    },
    core::{PCWSTR, w},
};

use crate::data::WindowsInfo;

struct RegKey(HKEY);
impl Drop for RegKey {
    fn drop(&mut self) {
        unsafe {
            let _ = RegCloseKey(self.0);
        }
    }
}

#[derive(Debug, Error)]
pub enum ReqKeyErr {
    #[error("failed to open key: {0}")]
    FailedToOpenKey(&'static str),
    #[error("failed to read key '{0}': {1}")]
    Win32Error(&'static str, windows::core::Error),
    #[error("unexpected type for key '{0}'")]
    InvalidType(&'static str),
}

fn read_string(hkey: HKEY, name: PCWSTR, name_debug: &'static str) -> Result<String, ReqKeyErr> {
    let mut size = 0u32;
    let mut ty = REG_VALUE_TYPE(0);

    unsafe {
        RegQueryValueExW(hkey, name, None, Some(&mut ty), None, Some(&mut size))
            .ok()
            .map_err(|e| ReqKeyErr::Win32Error(name_debug, e))?;
    }

    if ty != REG_SZ {
        return Err(ReqKeyErr::InvalidType(name_debug));
    }

    let mut buf = vec![0u16; (size / 2) as usize];
    unsafe {
        RegQueryValueExW(
            hkey,
            name,
            None,
            None,
            Some(buf.as_mut_ptr() as *mut u8),
            Some(&mut size),
        )
        .ok()
        .map_err(|e| ReqKeyErr::Win32Error(name_debug, e))?;
    }

    let s = String::from_utf16_lossy(&buf)
        .trim_matches(char::from(0))
        .to_string();
    Ok(s)
}

fn read_dword(hkey: HKEY, name: PCWSTR, name_debug: &'static str) -> Result<u32, ReqKeyErr> {
    let mut value = 0u32;
    let mut size = std::mem::size_of::<u32>() as u32;
    let mut ty = REG_VALUE_TYPE(0);

    unsafe {
        RegQueryValueExW(
            hkey,
            name,
            None,
            Some(&mut ty),
            Some(&mut value as *mut u32 as *mut u8),
            Some(&mut size),
        )
        .ok()
        .map_err(|e| ReqKeyErr::Win32Error(name_debug, e))?;
    }

    if ty != REG_DWORD {
        return Err(ReqKeyErr::InvalidType(name_debug));
    }

    Ok(value)
}

pub fn get_windowsnt_infomation() -> Result<WindowsInfo, ReqKeyErr> {
    let mut hkey_raw = HKEY::default();

    unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"),
            Some(0),
            KEY_READ,
            &mut hkey_raw,
        )
        .ok()
        .map_err(|_| ReqKeyErr::FailedToOpenKey("Windows NT CurrentVersion"))?;
    }

    let hkey = RegKey(hkey_raw);
    Ok(WindowsInfo {
        current_build: read_string(hkey.0, w!("CurrentBuild"), "CurrentBuild")?,
        current_major_version_number: read_dword(
            hkey.0,
            w!("CurrentMajorVersionNumber"),
            "CurrentMajorVersionNumber",
        )?,
        current_minor_version_number: read_dword(
            hkey.0,
            w!("CurrentMinorVersionNumber"),
            "CurrentMinorVersionNumber",
        )?,
        current_version: read_string(hkey.0, w!("CurrentVersion"), "CurrentVersion")?,
        display_version: read_string(hkey.0, w!("DisplayVersion"), "DisplayVersion")?,
        edition_id: read_string(hkey.0, w!("EditionID"), "EditionID")?,
        product_name: read_string(hkey.0, w!("ProductName"), "ProductName")?,
    })
}
