use crate::prelude::*;
use std::collections::HashMap;
use std::string::ToString;
use std::sync::RwLock;
#[derive(Debug, Clone)]

pub struct UserInfo {
    pub(crate) username: String,
}
impl UserInfo {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

// Define a ThreadLocal storage for user info
thread_local! {
    static Application_Info: RwLock<HashMap<String, UserInfo>> = RwLock::new(HashMap::new());
}

const USER_INFO: &str = "user_info";

pub fn storeUserInfo(username: &str) -> Result<()> {
    let user_info = UserInfo::new(username.to_string());
    Application_Info.with(|map| match map.write() {
        Ok(mut map) => {
            map.insert(USER_INFO.to_string(), user_info.clone());
            Ok(())
        }
        Err(_) => Err(crate::error::MyError::NotLoggedIn),
    })
}

pub fn getUserInfo() -> crate::prelude::Result<UserInfo> {
    Application_Info.with(|map| match map.read() {
        Ok(map) => match map.get(USER_INFO) {
            Some(user_info) => Ok(user_info.clone()),
            None => Err(crate::error::MyError::NotLoggedIn),
        },
        Err(_) => Err(crate::error::MyError::NotLoggedIn),
    })
}
