use crate::prelude::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encodeToken(sub: &str) -> Result<String> {
    let _date = Utc::now() + Duration::hours(100);
    let claim = Claims {
        sub: sub.to_owned(),
        exp: _date.timestamp() as usize,
    };
    let binding = std::env::var("SECRET_KEY")?;
    let secret_key = binding.as_bytes();

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret_key),
    )
    .map_err(|_| MyError::JWTEncodeError("Invalid".to_string()))
}

pub fn decodeToken(token: &str) -> Result<String> {
    info!("decoding token {token}");
    let binding = std::env::var("SECRET_KEY")?;
    let secret_key = binding.as_bytes();
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claim) => {
            if expired(claim.claims.exp) {
                error!("Token expired {}", claim.claims.exp);
                Err(MyError::JWTDecodeError("Token expired".to_string()))
            } else {
                Ok(claim.claims.sub)
            }
        }
        Err(_) => Err(MyError::JWTDecodeError("Invalid".to_string())),
    }
}

pub fn expired(exp: usize) -> bool {
    Utc::now().timestamp() as usize > exp
}
