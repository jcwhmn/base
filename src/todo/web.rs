use std::sync::Arc;
use actix_web::{get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path, ServiceConfig};
use log::{info, trace};
use crate::model::db::{AppState};
use crate::todo::model::TodoPatch;
use crate::todo::repo::TodoRepo;

#[get("/todos")]
pub async fn todo_list(db: Data<AppState>) -> impl  Responder {
    match TodoRepo::todo_list(&db.db).await {
        Ok(todos) =>
            HttpResponse::Ok().json(todos)
        ,
        Err(_) => HttpResponse::Ok().body(format!("No todos"))
    }
}

#[get("/todos/{id}")]
pub async fn todo_get(db: Data<AppState>, id: Path<i64>) -> impl Responder {
    let id = id.into_inner();
    match TodoRepo::todo_get(&db.db, id).await {
        Ok(todo) =>
            HttpResponse::Ok().json(todo)
        ,
        Err(_) => HttpResponse::NotFound().body("can't find")
    }
}

#[post("todos")]
pub async fn todo_create(db: Data<AppState>, param_obj: Json<TodoPatch>) -> impl Responder {
    let patch: TodoPatch = param_obj.into_inner();
    info!("Create todo with {:?}", patch);
    match TodoRepo::todo_create(&db.db, patch).await {
        Ok(todo) =>
            HttpResponse::Ok().json(todo)
        ,
        Err(_) => HttpResponse::InternalServerError().body("can't create")
    }
}

#[patch("todos/{id}")]
async fn todo_update(params_obj: Json<TodoPatch>, _db: Data<AppState>, id: Path<i64>) -> impl  Responder {
    let id = id.into_inner();
    let patch: TodoPatch = params_obj.into_inner();
    trace!("update todo with {:?}", patch);
    match TodoRepo::todo_update(&_db.db, id, patch).await {
        Ok(todo) =>
            HttpResponse::Ok().json(todo)
        ,
        Err(_) => HttpResponse::InternalServerError().body("can't update")
    }
}

pub fn todo_config(cfg: &mut ServiceConfig) {
    cfg.service(todo_list)
        .service(todo_get)
        .service(todo_create)
        .service(todo_update)
    ;
}
