use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::model::bet_info::BetInfo;
use crate::AppState;
use crate::Result;

#[axum::debug_handler]
pub async fn bet(
    State(state): State<Arc<AppState>>, 
    Json(bet_info): Json<BetInfo>
) -> Result<()> {
    let mut check_room_lock = state.room.lock().unwrap();

    if let Some(ref mut room) = *check_room_lock {
        room.bet_infos.push(bet_info.clone());
        Ok(())
    } else {
        Err(anyhow::anyhow!("No active room found").into())
    }
}
