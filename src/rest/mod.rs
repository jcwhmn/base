#[allow(unused_imports)]
pub mod application_info;
pub mod jwt;
mod middleware;

use std::sync::Arc;
use crate::model::db::{init_db, AppState};
use crate::rest::middleware::Auth;
use crate::todo::web::todo_config;
use crate::user::web::user_config;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::rest::application_info::get_user_info;

#[get("/")]
async fn health() -> impl Responder {
    let username = get_user_info().unwrap();
    HttpResponse::Ok().body(format!("Hello, {}", username.username))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    info!("Start server: http://localhost:8080");
    let db = init_db().await;
    info!("db = {:?}", db);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: Arc::new(db.clone())}))
            .wrap(Auth)
            .service(health)
            .service(
                web::scope("/api")
                    .configure(todo_config)
                    .configure(user_config),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
