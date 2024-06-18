use std::sync::Arc;

use axum::{debug_handler, extract::State, Json};
use chrono::{NaiveDateTime, TimeZone};
use serde::Deserialize;

use crate::{api::extractor::JWTAdminClaims, config, model::room::Room, AppState, Error, Result};

#[derive(Deserialize)]
pub struct CreateRoomData {
    name: String,
    max_participant_count: u32,
    start: NaiveDateTime,
}

#[debug_handler]
pub async fn create_room(
    State(state): State<Arc<AppState>>,
    _: JWTAdminClaims,
    Json(data): Json<CreateRoomData>,
) -> Result<()> {
    let start = config::TIME_ZONE
        .from_local_datetime(&data.start)
        .single()
        .ok_or_else(|| Error::Other(anyhow::anyhow!("Invalid end datetime")))?;

    let new_room = Room {
        name: data.name,
        max_participant_count: data.max_participant_count,
        start,
        bet_infos: Vec::new(),
    };

    let mut room_lock = state.room.lock().unwrap();
    *room_lock = Some(new_room);

    Ok(())
}
