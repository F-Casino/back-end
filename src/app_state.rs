use sqlx::SqlitePool;

use crate::config;

const SCHEMA: &str = include_str!("../schema.sql");

#[derive(Debug)]
pub struct AppState {
    pub database: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(config::DATABASE_URL).await.unwrap();

        sqlx::query(SCHEMA).execute(&pool).await.unwrap();

        Self { database: pool }
    }
}
