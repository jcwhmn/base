use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use futures::future::{err, ok, Ready};
use crate::model::ThisError;

pub struct UserCtx {
    pub user_id: i64,
}

impl FromRequest for UserCtx {
    type Error = actix_web::Error;
    type Future = Ready<Result<UserCtx, actix_web::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Some(token) = req.match_info().get(HEADER_XAUTH) {
            match token.parse::<i64>() {
                Ok(user_id) => ok(UserCtx { user_id: user_id}),
                Err(_) => err(ErrorUnauthorized("invalid token!")),
            }
        } else {
            err(ErrorUnauthorized("no token!"))
        }
    }

}

const HEADER_XAUTH: &str = "X-Auth-Token";

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Invalid Token {0}")]
    InvalidToken(String),
}


#[derive(Debug)]
pub struct FailAuth;
