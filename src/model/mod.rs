pub use thiserror::Error as ThisError;

pub mod db;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Entity not found - {0}[{1}] ")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}