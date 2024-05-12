use actix_web::http::header::ToStrError;
#[allow(unused_imports)]
use actix_web::ResponseError;
pub use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum MyError {
    // db errors
    #[error("Entity not found - {0}[{1}] ")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ToStrError(#[from] ToStrError),

    #[error(transparent)]
    EnvValError(#[from] std::env::VarError),

    // security errors
    #[error("Invalid Token {0}")]
    InvalidToken(String),

    #[error(transparent)]
    FailAuth(#[from] actix_web::error::Error),

    #[error("Not logged in")]
    NotLoggedIn,

    #[error("Authentication is not existed")]
    NoAuthorization,

    #[error("Authentication format is wrong")]
    WrongAuthorizationFormat,

    #[error("Jwt secret key must be set")]
    JWTSecretKeyError,

    #[error("Encode jwt error {0}")]
    JWTEncodeError(String),

    #[error("Decode jwt error {0}")]
    JWTDecodeError(String),
}
impl ResponseError for MyError {}
