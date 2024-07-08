use std::sync::Mutex;

use crate::model::room::Room;

#[derive(Debug)]
pub struct AppState {
    pub room: Mutex<Option<Room>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            room: Mutex::new(None),
        }
    }
}
