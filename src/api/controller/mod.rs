pub mod admin;
pub mod room;
pub mod user;

pub async fn ping() -> &'static str {
    "Pong"
}
