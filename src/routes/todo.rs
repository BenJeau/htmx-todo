use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, put},
    Router,
};

use crate::{state::AppState, templating};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", delete(delete_todo))
        .route("/done", put(done_todo))
        .route("/undone", put(undone_todo))
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<usize>) -> impl IntoResponse {
    state.todos.write().await.retain(|todo| todo.idx != id);
    state.alert_stats()
}

async fn done_todo(State(state): State<AppState>, Path(id): Path<usize>) -> impl IntoResponse {
    set_todo_done(state, id, true).await
}

async fn undone_todo(State(state): State<AppState>, Path(id): Path<usize>) -> impl IntoResponse {
    set_todo_done(state, id, false).await
}

async fn set_todo_done(state: AppState, id: usize, done: bool) -> impl IntoResponse {
    let mut todos = state.todos.write().await;

    if let Some(todo) = todos.iter_mut().find(|todo| todo.idx == id) {
        todo.done = done;
        state.alert_stats();
        return templating::TodoTemplate { todo: todo.clone() }.into_response();
    }

    ().into_response()
}
