use axum::{extract::State, routing::get, Router};
use tower_http::services::ServeDir;

use crate::{state::AppState, templating};

mod stats;
mod todo;
mod todos;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_index))
        .nest("/todos", todos::router().nest("/:id", todo::router()))
        .route("/stats", get(stats::sse_stats))
        .nest_service("/static", ServeDir::new("public/static"))
        .with_state(state)
}

async fn serve_index(State(state): State<AppState>) -> templating::IndexTemplate {
    templating::IndexTemplate {
        stats: state.get_stats().await,
        trust_user_input: state.trust_user_input,
    }
}
