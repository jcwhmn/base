#[allow(unused_imports)]
pub mod application_info;
pub mod jwt;
mod middleware;
mod middleware1;

use crate::model::db::{init_db, AppState};
use crate::rest::middleware::SayHi;
use crate::rest::middleware1::SayHi1;
use crate::todo::web::todo_config;
use crate::user::web::user_config;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("First page")
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    info!("Start server: http://localhost:8080");
    let db = init_db().await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db.clone() }))
            .wrap(SayHi1)
            .wrap(SayHi)
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
