use crate::{
    model::db::AppState,
    rest::{
        application_info::store_user_info,
        jwt::encodeToken,
    },
    todo::{
        model::TodoPatch,
        web::{todo_create, todo_get, todo_list, todo_update},
    },
    user::{
        model::{
            LoginRequest, RegisterUserRequest, CreateUser},
        postgres_user_repo::{UserRepo,PostgresUserRepo}
    }
};
use actix_web::{
    middleware, post, put, HttpResponse, Responder,
    web::{Data, Json, ServiceConfig},
};

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use log::info;
use std::time::Duration;

#[post("login")]
pub async fn user_login(user_repo: Box<dyn UserRepo>, param_obj: Json<LoginRequest>) -> impl Responder {
    let data = param_obj.into_inner();
    let username = data.username.as_str();
    let password = data.password.as_str();
    info!("username = {}, password = {}", username, password);
    // todo: validate user in db
    let result = user_repo.find_by_username_and_password(username, password).await
        .map(|user| store_user_info(&user.username.to_string()));
    //let result = store_user_info(user.username.to_str());
    info!("login successed: {:?}", result);
    match result {
        Ok(_) => HttpResponse::Ok().body("login success"),
        Err(_) => HttpResponse::InternalServerError().body("can't create"),
    }
}

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(user_login)
        .service(user_register);
}

#[put("register")]
pub async fn user_register(user_repo: Box<dyn UserRepo>, param_obj: Json<RegisterUserRequest>) -> impl Responder {
    let request = param_obj.into_inner();
    let create_user = CreateUser::new(request);
    info!("create_user = {:?}", create_user);

    let res = user_repo.user_create(create_user).await;
    info!("res = {:?}", res);
    match res {
        Ok(_) => HttpResponse::Ok().body("register success"),
        Err(_) => HttpResponse::InternalServerError().body("can't create"),
    }
}
