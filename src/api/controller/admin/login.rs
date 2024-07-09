use axum::{response::Redirect, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::{api::extractor::JWTAdminClaims, config, Error, Result};

static ENCODING_KEY: Lazy<EncodingKey> =
    Lazy::new(|| EncodingKey::from_secret(config::JWT_SECRET.as_bytes()));

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub async fn login(jar: CookieJar, Json(data): Json<LoginData>) -> Result<(CookieJar, Redirect)> {
    // TODO: return jwt token of admin if the username and password is right
    if data.username != config::ADMIN_USERNAME || data.password != config::ADMIN_PASSWORD {
        return Err(Error::Unauthorized {
            message: "Username or password is incorrect".to_string(),
        });
    }

    let exp = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as u64;

    let claims = JWTAdminClaims {
        sub: data.username.clone(),
        exp,
    };

    let token = encode(&Header::default(), &claims, &ENCODING_KEY)?;

    Ok((
        jar.add(Cookie::new("token", token)),
        Redirect::to("/admin")
    ))
}
