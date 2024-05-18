use std::ops::Deref;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use async_trait::async_trait;
use futures::future::{ok, err, Ready};
use futures::StreamExt;
use log::{error, info};
use sqlb::HasFields;

use crate::{
  model::db::Db, 
  prelude::*
};
use crate::model::db::AppState;
use crate::rest::application_info::store_user_info;

use super::{
  model::{CreateUser, User}
};

#[async_trait]
pub trait UserRepo {
  async fn user_create(&self, data: CreateUser) -> Result<User>;
  async fn find_by_username_and_password(&self, username: &str, password: &str) -> Result<User>;
  async fn find_by_username(&self, username: &str) -> Result<bool>;
  //fn user_update(&self, data: UserPatch) -> Result<()>;
}

#[derive(Clone)]
pub struct PostgresUserRepo {
  pub(crate) db: Db,
}

impl PostgresUserRepo {
  const TABLE: &'static str = "user";
  const COLUMNS: &'static [&'static str] = &["id", "username", "password", "email", "created_at", "created_by", "modified_at", "modified_by"];

  pub fn new(db: Db) -> Self {
      Self { db }
  }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
  async fn user_create(&self, data: CreateUser) -> Result<User> {
    let fields = data.not_none_fields();
    let sb = sqlb::insert()
        .table(Self::TABLE)
        .data(fields)
        .returning(Self::COLUMNS);
    // execute the query
    let user: User = sb.fetch_one(&self.db).await?;
    Ok(user)
  }

  async fn find_by_username_and_password(&self, username: &str, password: &str) -> Result<User> {
    let sb = sqlb::select()
        .table(Self::TABLE)
        //.columns(Self::COLUMNS)
        .and_where_eq("username", username)
        .and_where_eq("password", password);
    let user: User = sb.fetch_one(&self.db).await?;
    info!("user = {:?}", user);
    store_user_info(&user.username);
    Ok(user)
  }

  async fn find_by_username(&self, username: &str) -> Result<bool> {
    let sb = sqlb::select()
        .table(Self::TABLE)
        //.columns(Self::COLUMNS)
        .and_where_eq("username", username);
    let res: Option<User> = sb.fetch_optional(&self.db).await?;
    Ok(res.is_some())
  }
}

impl FromRequest for Box<dyn UserRepo> {
  type Error = MyError;
  type Future = Ready<Result<Box<dyn UserRepo>>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    // Get the database connection pool from the request extensions
    let pool = req
        .app_data::<Data<AppState>>()
        .expect("No database connection pool found");
    //let repo = PostgresTodoRepo::new(pool.clone());
    ok(Box::new(PostgresUserRepo::new(pool.db.deref().clone())))

    //ok(Box::new(PostgresUserRepo::new(&pool.unwrap().db.deref().clone())))
  }
}
