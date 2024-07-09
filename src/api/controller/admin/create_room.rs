use std::sync::Arc;

use axum::{debug_handler, extract::State, Json};
use axum_extra::extract::CookieJar;
use chrono::{NaiveDateTime, TimeZone};
use serde::Deserialize;

use crate::{config, model::room::Room, AppState, Error, Result};

#[derive(Deserialize)]
pub struct CreateRoomData {
    name: String,
}

#[debug_handler]
pub async fn create_room(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(data): Json<CreateRoomData>,
) -> Result<()> {
    if jar.get("token").is_none() {
        return Err(Error::Unauthorized {
            message: "Invalid token".to_string(),
        });
    };

    let new_room = Room {
        name: data.name,
        bet_infos: Vec::new(),
    };

    let mut room_lock = state.room.lock().unwrap();
    *room_lock = Some(new_room);

    Ok(())
}
