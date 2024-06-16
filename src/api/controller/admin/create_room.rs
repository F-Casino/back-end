use std::sync::Arc;

use axum::{extract::State, Json, debug_handler};
use chrono::{NaiveDateTime, TimeZone};
use serde::Deserialize;

use crate::util::time;

use crate::{api::extractor::JWTAdminClaims, config, model::room::Room, AppState, Error, Result};

#[derive(Deserialize)]
pub struct CreateRoomData {
    name: String,
    max_participant_count: u32,
    end: NaiveDateTime,
}

#[debug_handler]
pub async fn create_room(
    State(state): State<Arc<AppState>>,
    _: JWTAdminClaims,
    Json(data): Json<CreateRoomData>,
) -> Result<()> {
    // TODO: implement create room
    // convert NaiveDateTime to DateTime<Tz> using config::TIME_ZONE.from_local_datetime(...)
    let end_datetime = config::TIME_ZONE
        .from_local_datetime(&data.end)
        .single()
        .ok_or_else(|| Error::Other(
            anyhow::anyhow!("Invalid end datetime")
        ))?;
    
    let new_room = Room {
        name: data.name,
        max_participant_count: data.max_participant_count,
        start: time::now(),
        end: end_datetime
    };

    let mut room_lock = state.room.lock().unwrap();
    *room_lock = Some(new_room);

    Ok(())
}
