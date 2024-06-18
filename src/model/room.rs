use chrono::DateTime;
use chrono_tz::Tz;
use serde::Serialize;

use super::bet_info::BetInfo;

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub name: String,
    pub max_participant_count: u32,
    pub start: DateTime<Tz>,
    #[serde(skip)]
    pub bet_infos: Vec<BetInfo>,
}
