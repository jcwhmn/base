use std::ops::Deref;
use crate::model::db::AppState;
use crate::todo::model::TodoPatch;
use crate::todo::postgres_todo_repo;
use crate::todo::postgres_todo_repo::PostgresTodoRepo;
use crate::todo::todo_repo::TodoRepo;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, patch, post, HttpResponse, Responder};
use log::{info, trace};

#[get("/todos")]
pub async fn todo_list(todo_repo: PostgresTodoRepo) -> impl Responder {
    match todo_repo.todo_list().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::Ok().body(format!("No todos")),
    }
}

#[get("/todos/{id}")]
pub async fn todo_get(todo_repo: PostgresTodoRepo, id: Path<i64>) -> impl Responder {
    //    let todo_repo = postgres_todo_repo::PostgresTodoRepo::new((&db.db).clone());
    let id = id.into_inner();
    match todo_repo.todo_get(id).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::NotFound().body("can't find"),
    }
}

#[post("todos")]
pub async fn todo_create(todo_repo: PostgresTodoRepo, param_obj: Json<TodoPatch>) -> impl Responder {
    let patch: TodoPatch = param_obj.into_inner();
    info!("Create todo with {:?}", patch);
    match todo_repo.todo_create(patch).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().body("can't create"),
    }
}

#[patch("todos/{id}")]
async fn todo_update(
    params_obj: Json<TodoPatch>,
    todo_repo: PostgresTodoRepo,
    id: Path<i64>,
) -> impl Responder {
    let id = id.into_inner();
    let patch: TodoPatch = params_obj.into_inner();
    trace!("update todo with {:?}", patch);
    match todo_repo.todo_update(id, patch).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().body("can't update"),
    }
}

pub fn todo_config(cfg: &mut ServiceConfig) {
    cfg.service(todo_list)
        .service(todo_get)
        .service(todo_create)
        .service(todo_update);
}
