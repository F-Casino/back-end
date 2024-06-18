use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::model::room::Room;
use crate::AppState;
use crate::Result;

pub async fn get(State(state): State<Arc<AppState>>) -> Result<Json<Option<Room>>> {
    let room = state
        .room
        .lock()
        .map_err(|err| anyhow::anyhow!(err.to_string()))?
        .clone();

    Ok(Json(room))
}
