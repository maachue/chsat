use uzers::{get_current_uid, get_current_username, get_user_by_uid, get_user_groups};

use crate::data::ser::UserInfo;

#[derive(Debug, thiserror::Error)]
pub enum UserInfoFailed {
    #[error("failed to get current user and current groups")]
    GetFailed,
}

pub fn get_unix_user_infomation() -> Result<UserInfo, UserInfoFailed> {
    let uid = get_current_uid();

    let mut gids = Vec::new();
    let mut groups_name = Vec::new();
    let username = get_current_username().ok_or(UserInfoFailed::GetFailed)?;
    if let Some(user) = get_user_by_uid(uid)
        && let Some(groups) = get_user_groups(user.name(), user.primary_group_id())
    {
        for group in groups {
            gids.push(group.gid().to_string());
            groups_name.push(group.name().to_string_lossy().into_owned());
        }

        Ok(UserInfo {
            uid: uid.to_string(),
            username: username.to_string_lossy().into_owned(),
            gids,
            groups: groups_name,
        })
    } else {
        Err(UserInfoFailed::GetFailed)
    }
}
