use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../../../admin.html"))
}
