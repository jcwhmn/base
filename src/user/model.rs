use std::time::Instant;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::rest::application_info::get_user_info;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_by: String,
    pub created_at: OffsetDateTime,
    pub modified_by: Option<String>,
    pub modified_at: Option<DateTime<chrono::Utc>>,
}

impl User {
    pub fn new(username: String, password: String, email: String) -> Self {
        Self {
            id: 0,
            username,
            password,
            email,
            created_by: get_user_info().unwrap().username,
            created_at: OffsetDateTime::now_utc(),
            modified_by: None,
            modified_at: None,
        }
    }

    pub fn update(&mut self, username: String, email: String) {
        self.username = username;
        self.email = email;
        self.modified_by = Some(get_user_info().unwrap().username);
        self.modified_at = Some(Utc::now());
    }

    pub fn update_password(&mut self, password: String) {
        self.password = password;
        self.modified_by = Some(get_user_info().unwrap().username);
        self.modified_at = Some(Utc::now());
    }
}

// regin db models
#[derive(Debug, Clone, sqlb::Fields)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_by: String,
    pub created_at: OffsetDateTime,
}

impl CreateUser {
    pub(crate) fn new(registerUser: RegisterUserRequest) -> Self {
        Self {
            username: registerUser.username,
            password: registerUser.password,
            email: registerUser.email,
            created_by: get_user_info().unwrap().username,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub modified_by: String,
    pub modified_at: OffsetDateTime,
}

impl UpdateUser {
    fn new(updateUser: UpdateUserRequest) -> Self {
        Self {
            username: updateUser.username,
            email: updateUser.email,
            modified_by: get_user_info().unwrap().username,
            modified_at: OffsetDateTime::now_utc(),
        }
    }
}

// region web models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: String,
}
// endregion