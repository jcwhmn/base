use async_trait::async_trait;

use crate::prelude::*;

use super::model::{CreateUser, LoginRequest, User};

#[async_trait]
pub trait UserRepo1 {
  async fn user_create(&self, data: CreateUser) -> Result<User>;
  async fn login(&self, username: &str, password: &str) -> Result<User>;
    //fn user_update(&self, data: UserPatch) -> Result<()>;
}