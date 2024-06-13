use axum::Json;
use serde::Deserialize;

use crate::Result;

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub async fn login(Json(data): Json<LoginData>) -> Result<String> {
    // TODO: return jwt token of admin if the username and password is right 
    
    todo!()
}
