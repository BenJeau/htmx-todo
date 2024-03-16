use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::templating;

pub fn router() -> Router {
    Router::new()
        .route("/", get(serve_index))
        .nest_service("/static", ServeDir::new("public/static"))
}

async fn serve_index() -> templating::IndexTemplate {
    templating::IndexTemplate
}
