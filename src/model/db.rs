use dotenv::dotenv;
use log::error;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub struct AppState {
    pub db: Db,
}

pub async fn init_db() -> Db {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(db) => db,
        Err(_) => {
            error!("Error connecting to database");
            std::process::exit(1);
        }
    }
}
