use crate::prelude::*;
use crate::security::UserCtx;
use crate::todo::model::{Todo, TodoPatch};
use async_trait::async_trait;

pub type TodoResult = Result<Todo>;
#[async_trait]
pub trait TodoRepo: Send + Sync + 'static {
    async fn todo_create(&self, data: TodoPatch) -> TodoResult;
    async fn todo_get(&self, id: i64) -> TodoResult;
    async fn todo_update(&self, id: i64, data: TodoPatch) -> TodoResult;
    async fn todo_list(&self) -> Result<Vec<Todo>>;
    async fn delete(&self, _utx: &UserCtx, id: i64) -> Result<u64>;
}
