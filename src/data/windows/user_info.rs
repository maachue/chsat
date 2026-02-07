use thiserror::Error;
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, HLOCAL, LocalFree},
        Security::{
            Authorization::ConvertSidToStringSidW, GetTokenInformation, LookupAccountSidW, PSID,
            SID_NAME_USE, TOKEN_GROUPS, TOKEN_QUERY, TOKEN_USER, TokenGroups, TokenUser,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    },
    core::PWSTR,
};

use crate::data::ser::UserInfo;

#[derive(Debug, Error)]
pub enum SecurityErr {
    #[error("win32 error {0}: {1}")]
    WIn32Error(&'static str, windows::core::Error),
    #[error("failed to convert from UTF-16 to UTF-8. Raw msg: {0}")]
    Utf16Convert(#[from] std::string::FromUtf16Error),
}

struct TokenHandle(HANDLE);

impl Default for TokenHandle {
    fn default() -> Self {
        Self(HANDLE::default())
    }
}

impl Drop for TokenHandle {
    fn drop(&mut self) {
        let _ /* Result<T, E> ignore */ = unsafe { CloseHandle(self.0) };
    }
}

struct OwnedPWSTR(PWSTR);

impl Default for OwnedPWSTR {
    fn default() -> Self {
        Self(PWSTR::null())
    }
}

impl TryFrom<PSID> for OwnedPWSTR {
    type Error = SecurityErr;

    fn try_from(sid: PSID) -> Result<Self, Self::Error> {
        unsafe {
            let mut string_sid = Self::default();

            ConvertSidToStringSidW(sid, &mut string_sid.0)
                .map_err(|e| SecurityErr::WIn32Error("convert from PSID to PWSTR", e))?;

            Ok(string_sid)
        }
    }
}

impl Drop for OwnedPWSTR {
    fn drop(&mut self) {
        unsafe {
            if self.0 != PWSTR::null() {
                let free_me = HLOCAL(self.0.0 as *mut _); // *mut c_void
                let _ /* HLOCAL */ = LocalFree(Some(free_me));
            }
        }
    }
}

fn get_user_sid(token: &TokenHandle) -> Result<String, SecurityErr> {
    unsafe {
        let mut dw_size = 0;
        let _result = GetTokenInformation(token.0, TokenUser, None, 0, &mut dw_size);

        let mut buffer = vec![0u8; dw_size as usize];
        GetTokenInformation(
            token.0,
            TokenUser,
            Some(buffer.as_mut_ptr() as *mut _ /* *mut c_void */),
            dw_size,
            &mut dw_size,
        )
        .map_err(|e| SecurityErr::WIn32Error("get TOKEN_USER", e))?;
        let token_user = &*(buffer.as_ptr().cast::<TOKEN_USER>());
        let sid = token_user.User.Sid;

        let string_sid = OwnedPWSTR::try_from(sid)?;
        let output = string_sid.0.to_string()?; // copy data

        Ok(output)
    }
}

fn lookup_account_sid(sid: PSID) -> Result<(String, String), SecurityErr> {
    unsafe {
        let mut name_len = 0u32;
        let mut domain_len = 0u32;
        let mut use_type = SID_NAME_USE(0);

        let _ = LookupAccountSidW(
            None,
            sid,
            None,
            &mut name_len,
            None,
            &mut domain_len,
            &mut use_type,
        );

        let mut name_buf = vec![0u16; name_len as usize];
        let mut domain_buf = vec![0u16; domain_len as usize];

        LookupAccountSidW(
            None,
            sid,
            Some(PWSTR(name_buf.as_mut_ptr())),
            &mut name_len,
            Some(PWSTR(domain_buf.as_mut_ptr())),
            &mut domain_len,
            &mut use_type,
        )
        .map_err(|e| SecurityErr::WIn32Error("looking up SID", e))?;

        let name = String::from_utf16_lossy(&name_buf[..(name_len as usize)])
            .trim_end_matches('\0')
            .to_string();
        let domain = String::from_utf16_lossy(&domain_buf[..(domain_len as usize)])
            .trim_end_matches('\0')
            .to_string();

        Ok((domain, name))
    }
}

fn get_group_sid(token: &TokenHandle) -> Result<(Vec<String>, Vec<String>), SecurityErr> {
    unsafe {
        let mut dw_size = 0;
        let _result = GetTokenInformation(token.0, TokenGroups, None, 0, &mut dw_size);

        let mut buffer = vec![0u8; dw_size as usize];
        GetTokenInformation(
            token.0,
            TokenGroups,
            Some(buffer.as_mut_ptr() as *mut _ /* *mut c_void */),
            dw_size,
            &mut dw_size,
        )
        .map_err(|e| SecurityErr::WIn32Error("get TOKEN_GROUPS", e))?;
        let token_groups = &*(buffer.as_mut_ptr().cast::<TOKEN_GROUPS>());
        let groups = std::slice::from_raw_parts(
            token_groups.Groups.as_ptr(),
            token_groups.GroupCount as usize,
        );

        let mut vec_result = Vec::new();
        let mut vec_result1 = Vec::new();
        for group in groups {
            let sid = group.Sid;

            let (domain, name) = lookup_account_sid(sid)?;

            let string_sid = OwnedPWSTR::try_from(sid)?;
            let sid_as_string_rs = string_sid.0.to_string()?; // needed copy data

            vec_result.push(sid_as_string_rs);
            vec_result1.push(format!("{}\\{}", domain, name));
        }
        Ok((vec_result, vec_result1))
    }
}

pub fn windowsnt_get_user_info() -> Result<UserInfo, SecurityErr> {
    unsafe {
        let mut token = TokenHandle::default();

        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token.0)
            .map_err(|e| SecurityErr::WIn32Error("get Current Process token", e))?;

        let uid = get_user_sid(&mut token)?;
        let (gids, groups) = get_group_sid(&mut token)?;

        Ok(UserInfo {
            uid,
            username: whoami::username(),
            gids,
            groups,
        })
    }
}
