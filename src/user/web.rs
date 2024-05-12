use crate::model::db::AppState;
use crate::rest::application_info::storeUserInfo;
use crate::rest::jwt::encodeToken;
use crate::todo::model::TodoPatch;
use crate::todo::web::{todo_create, todo_get, todo_list, todo_update};
use crate::user::model::LoginRequest;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{middleware, post, HttpResponse, Responder};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use log::info;
use std::time::Duration;

#[post("login")]
pub async fn user_login(db: Data<AppState>, param_obj: Json<LoginRequest>) -> impl Responder {
    let username = param_obj.into_inner().username;
    let username = username.as_str();
    info!("username = {}", username);
    // todo: validate user in db

    let result = storeUserInfo(username);
    info!("login successed: {:?}", result);
    match result {
        Ok(_) => HttpResponse::Ok().body("login success"),
        Err(_) => HttpResponse::InternalServerError().body("can't create"),
    }
}

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(user_login);
}
