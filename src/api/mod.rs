use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;

use crate::config;
use crate::AppState;

mod controller;
mod extractor;
mod router;

pub async fn start_api() {
    let state = Arc::new(AppState::new().await);
    let app = router::build(state);
    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), config::SERVER_PORT))
        .await
        .unwrap();
    tracing::info!("Listening On Port: {}", config::SERVER_PORT);
    axum::serve(listener, app).await.unwrap();
}
