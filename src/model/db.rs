use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type Db = Pool<Postgres>;

pub struct AppState {
    pub db: Db,
}

pub async fn init_db() -> Result<Db, sqlx::Error> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
