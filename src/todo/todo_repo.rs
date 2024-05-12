use async_trait::async_trait;
use sqlb::HasFields;
use crate::model::db::Db;
use crate::prelude::*;
use crate::security::UserCtx;
use crate::todo::model::{Todo, TodoPatch};

pub type TodoResult = Result<Todo>;
#[async_trait]
pub trait TodoRepo: Send + Sync + 'static {
    async fn todo_create(&self, db: &Db, data: TodoPatch) -> TodoResult;
    async fn todo_get(&self, db: &Db, id: i64) -> TodoResult;
    async fn todo_update(&self, db: &Db, id: i64, data: TodoPatch) -> TodoResult;
    async fn todo_list(&self, db: &Db) -> TodoResult;
    async fn delete(&self, db: &Db, _utx: &UserCtx, id: i64) -> Result<u64>;

    fn new() -> impl TodoRepo {
        PostgresTodoRepo.new()
    }
}

//regin: TodoMac
pub struct PostgresTodoRepo;

impl PostgresTodoRepo {
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];

    fn new() -> Self {
        Self
    }
}


#[async_trait]
impl TodoRepo for PostgresTodoRepo {
    async fn todo_create(&self, db: &Db, data: TodoPatch) -> Result<Todo> {
        let mut fields = data.not_none_fields();
        fields.push(("cid", 123).into());
        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);
        // execute the query
        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    async fn todo_get(&self, db: &Db, id: i64) -> Result<Todo> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let todo = sb
            .fetch_one(db).await
            .map_err(|error|match error {
                sqlx::Error::RowNotFound => Error::EntityNotFound(Self::TABLE, id.to_string()),
                other => Error::SqlxError(other)
            })?;

        Ok(todo)
    }

    async fn todo_update(&self, db: &Db, id: i64, data: TodoPatch) -> Result<Todo> {
        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let todo = sb.fetch_one(db).await?;
        Ok(todo)
    }

    async fn todo_list(&self, db: &Db) -> Result<Vec<Todo>> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("id");

        let todos = sb
            .fetch_all(db).await?;

        Ok(todos)
    }


    async fn delete(&self, db: &Db, _utx: &UserCtx, id: i64) -> Result<u64> {
        let sb = sqlb::delete()
            .table(Self::TABLE)
            .and_where_eq("id", id);

        let num = sb
            .exec(db).await
            .map_err(|error|match error {
                sqlx::Error::RowNotFound => Error::EntityNotFound(Self::TABLE, id.to_string()),
                other => Error::SqlxError(other)
            })?;

        Ok(num)
    }

}
//endregion: TodoMac
