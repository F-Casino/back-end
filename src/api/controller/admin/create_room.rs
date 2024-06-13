use axum::Json;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::Result;

#[derive(Deserialize)]
pub struct CreateRoomData {
    name: String,
    max_participant_count: u32,
    end: NaiveDateTime,
}

pub async fn create_room(Json(data): Json<CreateRoomData>) -> Result<()> {
    // TODO: implement create room
    // convert NaiveDateTime to DateTime<Tz> using config::TIME_ZONE.from_local_datetime(...)
    Ok(())
}
