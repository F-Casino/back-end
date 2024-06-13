use std::sync::Mutex;

use sqlx::SqlitePool;

use crate::config;
use crate::model::room::Room;

const SCHEMA: &str = include_str!("../schema.sql");

#[derive(Debug)]
pub struct AppState {
    pub database: SqlitePool,
    pub room: Mutex<Option<Room>>,
}

impl AppState {
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(config::DATABASE_URL).await.unwrap();

        sqlx::query(SCHEMA).execute(&pool).await.unwrap();

        Self {
            database: pool,
            room: Mutex::new(None),
        }
    }
}
