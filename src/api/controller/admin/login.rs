use axum::Json;
use once_cell::sync::Lazy;
use serde::Deserialize;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{api::extractor::JWTAdminClaims, config, Error, Result};

static ENCODING_KEY: Lazy<EncodingKey> = 
    Lazy::new(|| EncodingKey::from_secret(config::JWT_SECRET.as_bytes()));

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub async fn login(Json(data): Json<LoginData>) -> Result<String> {
    // TODO: return jwt token of admin if the username and password is right 
    if data.username != config::ADMIN_USERNAME || data.password != config::ADMIN_PASSWORD {
        return Err(Error::Unauthorized{
            message: "Username or password is incorrect".to_string(),
        });
    }
    let claims = JWTAdminClaims {
        sub: data.username.clone(),
        exp: 60000,
    };

    let token = encode(&Header::default(), &claims, &ENCODING_KEY);

    Ok(token?)
}
