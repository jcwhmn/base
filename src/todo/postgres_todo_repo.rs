use std::ops::Deref;
use std::sync::Arc;
use crate::error::MyError;
use crate::model::db::{AppState, Db};
use crate::prelude::*;
use crate::security::UserCtx;
use crate::todo::model::{Todo, TodoPatch};
use crate::todo::todo_repo::TodoRepo;
use actix_web::{
    FromRequest, HttpRequest,
    dev::Payload
};
use actix_web::web::Data;
use async_trait::async_trait;
use futures::future::{ok, Ready};
use futures::StreamExt;
use log::info;
use sqlb::HasFields;

#[derive(Clone)]
pub(crate) struct PostgresTodoRepo {
    pub(crate) db: Db,
}

impl PostgresTodoRepo {
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];

    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepo for PostgresTodoRepo {
    async fn todo_create(&self, data: TodoPatch) -> crate::prelude::Result<Todo> {
        let mut fields = data.not_none_fields();
        fields.push(("cid", 123).into());
        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);
        // execute the query
        let todo = sb.fetch_one(&self.db).await?;

        Ok(todo)
    }

    async fn todo_get(&self, id: i64) -> crate::prelude::Result<Todo> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let todo = sb.fetch_one(&self.db).await.map_err(|error| match error {
            sqlx::Error::RowNotFound => MyError::EntityNotFound(Self::TABLE, id.to_string()),
            other => MyError::SqlxError(other),
        })?;

        Ok(todo)
    }

    async fn todo_update(&self, id: i64, data: TodoPatch) -> crate::prelude::Result<Todo> {
        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let todo = sb.fetch_one(&self.db).await?;
        Ok(todo)
    }

    async fn todo_list(&self) -> crate::prelude::Result<Vec<Todo>> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("id");

        let todos = sb.fetch_all(&self.db).await?;

        Ok(todos)
    }

    async fn delete(&self, _utx: &UserCtx, id: i64) -> crate::prelude::Result<u64> {
        let sb = sqlb::delete().table(Self::TABLE).and_where_eq("id", id);

        let num = sb.exec(&self.db).await.map_err(|error| match error {
            sqlx::Error::RowNotFound => MyError::EntityNotFound(Self::TABLE, id.to_string()),
            other => MyError::SqlxError(other),
        })?;

        Ok(num)
    }
}

impl FromRequest for PostgresTodoRepo {
    type Error = MyError;
    type Future = Ready<Result<PostgresTodoRepo>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get the database connection pool from the request extensions
        //req.app_data::<AppState>();
        let pool = req
            .app_data::<Data<AppState>>()
            .expect("No database connection pool found");
        //let repo = PostgresTodoRepo::new(pool.clone());
        ok(PostgresTodoRepo::new(pool.db.deref().clone()))
    }
}
