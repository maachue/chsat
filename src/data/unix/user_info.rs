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

use uzers::{get_current_uid, get_user_by_uid, get_user_groups};

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
    if let Some(user) = get_user_by_uid(uid)
        && let Some(groups) = get_user_groups(user.name(), user.primary_group_id())
    {
        for group in groups {
            gids.push(group.gid().to_string());
            groups_name.push(group.name().to_string_lossy().into_owned());
        }

        Ok(UserInfo {
            uid: uid.to_string(),
            username: whoami::username(),
            gids,
            groups: groups_name,
        })
    } else {
        Err(UserInfoFailed::GetFailed)
    }
}
