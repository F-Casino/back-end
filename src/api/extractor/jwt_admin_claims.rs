use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{config, Error};

static DECODING_KEY: Lazy<DecodingKey> =
    Lazy::new(|| DecodingKey::from_secret(config::JWT_SECRET.as_bytes()));

#[derive(Serialize, Deserialize)]
pub struct JWTAdminClaims {
    /// username of admin
    pub sub: String,
    pub exp: u64,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for JWTAdminClaims {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token();

        let token_data = decode::<JWTAdminClaims>(token, &DECODING_KEY, &Validation::default())?;

        let claims = token_data.claims;

        if claims.sub != config::ADMIN_USERNAME {
            return Err(Error::Unauthorized {
                message: "Only admin can access this".to_string(),
            });
        }

        Ok(claims)
    }
}
