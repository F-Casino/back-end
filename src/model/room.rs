use chrono::DateTime;
use chrono_tz::Tz;
use serde::Serialize;

use crate::util::time;

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub name: String,
    pub max_participant_count: u32,
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
}

impl Room {
    fn new(name: String, max_participant_count: u32, end: DateTime<Tz>) -> Room {
        Room {
            name,
            max_participant_count,
            start: time::now(),
            end,
        }
    }
}
