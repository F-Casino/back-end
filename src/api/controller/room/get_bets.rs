use std::sync::Arc;

use axum::extract::State;
use axum::response::Html;
use axum::Json;
use sailfish::TemplateOnce;

use crate::model::bet_info::BetInfo;
use crate::model::room::Room;
use crate::AppState;
use crate::Result;

#[axum::debug_handler]
pub async fn get_bets(State(state): State<Arc<AppState>>) -> Result<Json<Option<Vec<BetInfo>>>> {
    let bet_infos = state
        .room
        .lock()
        .map_err(|err| anyhow::anyhow!(err.to_string()))?
        .clone()
        .map(|room| room.bet_infos);

    Ok(Json(bet_infos))
}
