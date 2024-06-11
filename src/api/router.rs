use axum::http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use super::controller;
use crate::{app_state::AppState, config};

const ALLOW_HEADERS: [HeaderName; 7] = [
    ORIGIN,
    AUTHORIZATION,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
    ACCEPT,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_HEADERS,
];
const ALLOW_METHODS: [Method; 2] = [Method::GET, Method::POST];

pub fn build(state: Arc<AppState>) -> Router {
    let allow_origins = [
        config::PUBLIC_CORS_DOMAIN.parse::<HeaderValue>().unwrap(),
        config::LOCAL_CORS_DOMAIN.parse::<HeaderValue>().unwrap(),
    ];

    // register routes
    let router = Router::new().route("/", get(controller::ping));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http()).layer(
        CorsLayer::new()
            .allow_origin(allow_origins)
            .allow_headers(ALLOW_HEADERS)
            .expose_headers(ALLOW_HEADERS)
            .allow_credentials(true)
            .allow_methods(ALLOW_METHODS),
    );

    // init state
    router.with_state(state)
}
