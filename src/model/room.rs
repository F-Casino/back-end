use serde::Serialize;

use super::bet_info::BetInfo;

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub name: String,
    pub bet_infos: Vec<BetInfo>,
}
