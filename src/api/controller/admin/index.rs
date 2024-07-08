use crate::Result;
use axum::response::Html;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index {}

pub async fn index() -> Result<Html<String>> {
    let index = Index {}
        .render_once()
        .map(Html)
        .map_err(|error| anyhow::anyhow!(error))?;

    Ok(index)
}
