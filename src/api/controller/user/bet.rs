use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::model::bet_info::BetInfo;
use crate::AppState;
use crate::Result;

#[axum::debug_handler]
pub async fn bet(State(state): State<Arc<AppState>>, Json(bet_info): Json<BetInfo>) -> Result<()> {
    todo!()
}
