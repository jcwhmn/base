use std::time::Instant;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::rest::application_info::getUserInfo;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub creater_id: String,
    pub creater_at: NaiveDateTime,
    pub updater_id: Option<String>,
    pub updater_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(id: i32, username: String, password: String, email: String) -> Self {
        Self {
            id,
            username,
            password,
            email,
            creater_id: getUserInfo().unwrap().username,
            creater_at: Utc::now().naive_local(),
            updater_id: None,
            updater_at: None,
        }
    }

    pub fn update(&mut self, username: String, email: String) {
        self.username = username;
        self.email = email;
        self.updater_id = Some(getUserInfo().unwrap().username);
        self.updater_at = Some(Utc::now().naive_local());
    }

    pub fn update_password(&mut self, password: String) {
        self.password = password;
        self.updater_id = Some(getUserInfo().unwrap().username);
        self.updater_at = Some(Utc::now().naive_local());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub username: String,
    pub password: String,
    pub email: String,
}
