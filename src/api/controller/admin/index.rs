use crate::Result;
use axum::response::Html;
use axum_extra::extract::CookieJar;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index {
    is_authorized: bool,
}

pub async fn index(jar: CookieJar) -> Result<Html<String>> {
    let is_authorized = jar.get("token").is_some();
    let index = Index { is_authorized }
        .render_once()
        .map(Html)
        .map_err(|error| anyhow::anyhow!(error))?;

    Ok(index)
}
