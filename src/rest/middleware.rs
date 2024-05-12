use crate::error::MyError::*;
use crate::prelude;
use crate::prelude::MyError::{NoAuthorization, WrongAuthorizationFormat};
use crate::rest::application_info::{getUserInfo, storeUserInfo, UserInfo};
use crate::rest::jwt::{decodeToken, encodeToken};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
use actix_web::Error;
use log::{error, info};
#[warn(unused_imports)]
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::str::FromStr;

pub struct SayHi;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let result = process_jwt(&req);
        if result.is_err() {
            error!("invalid token!");
            return Box::pin(ready(Err(result.err().unwrap().into())));
        }
        // A more complex middleware, could return an error or an early response here.

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            post_response(res.headers_mut());
            println!("Hi from response");
            Ok(res)
        })
    }
}

const BEARER: &str = "Bearer";

fn post_response(map: &mut HeaderMap) {
    // fetch UserInfo from ThreadLocal
    let user_info = getUserInfo();
    if user_info.is_err() {
        return;
    }
    let username = user_info.unwrap().username;

    // encode token from username
    let token = format!("{} {}", BEARER, encodeToken(&username).unwrap());

    // set token to Authorization header
    map.insert(
        HeaderName::from_str("Authorization").unwrap(),
        HeaderValue::from_str(&token).unwrap(),
    );
}

fn process_jwt(req: &ServiceRequest) -> prelude::Result<()> {
    if req.path().find("/api/login") == Some(0) {
        return Ok(());
    }

    let auth = req.headers().get("Authorization").ok_or(NoAuthorization)?;
    let auths: Vec<&str> = auth.to_str()?.split(" ").collect();
    info!("auths = {auths:?}");
    if auths.len() != 2 || auths[0] != BEARER {
        error!("not authorized");
        return Err(WrongAuthorizationFormat);
    }
    let username = decodeToken(auths[1])?;
    storeUserInfo(&username)?;
    info!("auth = {:?}", auth);
    Ok(())
}
