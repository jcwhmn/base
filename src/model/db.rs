use std::sync::Arc;
use actix_web::rt::Runtime;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::{error, info};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<Db>,
}

pub async fn init_db() -> Db {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(db) => {
            info!("Db Initialized");
            db
        },
        Err(_) => {
            error!("Error connecting to database");
            std::process::exit(1);
        }
    }
}
