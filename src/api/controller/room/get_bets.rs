use std::sync::Arc;

use axum::extract::State;
use axum::response::Html;
use axum::Json;
use sailfish::TemplateOnce;

use crate::model::bet_info::BetInfo;
use crate::model::room::Room;
use crate::AppState;
use crate::Result;

#[derive(TemplateOnce)]
#[template(path = "bet_infos.stpl")]
struct BetInfos {
    data: Vec<BetInfo>,
}

#[axum::debug_handler]
pub async fn get_bets(State(state): State<Arc<AppState>>) -> Result<Html<String>> {
    let bet_infos = state
        .room
        .lock()
        .map_err(|err| anyhow::anyhow!(err.to_string()))?
        .clone()
        .map(|room| BetInfos {
            data: room.bet_infos,
        })
        .map(|bet_infos| bet_infos.render_once())
        .ok_or(anyhow::anyhow!("No room"))?
        .map_err(anyhow::Error::from)?;

    Ok(Html(bet_infos))
}
