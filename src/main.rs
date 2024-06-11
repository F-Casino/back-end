mod error;
mod api;
mod config;
mod app_state;
mod model;
mod util;

pub use error::*;
pub use app_state::*;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
        )
        .init();
    api::start_api().await;
}
