use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::{state::AppState, templating};

mod todos;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_index))
        .nest("/todos", todos::router())
        .nest_service("/static", ServeDir::new("public/static"))
        .with_state(state)
}

async fn serve_index() -> templating::IndexTemplate {
    templating::IndexTemplate
}
