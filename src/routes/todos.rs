use axum::{
    extract::State,
    response::IntoResponse,
    routing::{delete, get, put},
    Form, Router,
};
use serde::Deserialize;

use crate::{
    state::{AppState, Todo},
    templating,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_todo).post(add_todo).delete(delete_all_todos))
        .route("/done", delete(delete_done_todos).put(mark_all_done))
        .route("/undone", put(mark_all_undone))
}

async fn get_todo(State(state): State<AppState>) -> templating::TodosTemplate {
    templating::TodosTemplate {
        todos: state.todos.read().await.to_vec(),
    }
}

#[derive(Deserialize)]
struct AddTodo {
    description: String,
}

async fn add_todo(State(state): State<AppState>, Form(todo): Form<AddTodo>) -> impl IntoResponse {
    let mut todos = state.todos.write().await;

    *state.id_counter.write().await += 1;
    let todo = Todo {
        idx: *state.id_counter.read().await,
        done: false,
        description: todo.description,
    };
    todos.push(todo.clone());

    state.alert_stats();

    templating::AddTodoTemplate { todo }
}

async fn delete_all_todos(State(state): State<AppState>) -> impl IntoResponse {
    state.todos.write().await.clear();
    state.alert_stats();

    templating::TodosTemplate::default()
}

async fn delete_done_todos(State(state): State<AppState>) -> impl IntoResponse {
    let mut todos = state.todos.write().await;

    todos.retain(|todo| !todo.done);
    state.alert_stats();

    templating::TodosTemplate {
        todos: todos.to_vec(),
    }
}

async fn mark_all_done(State(state): State<AppState>) -> impl IntoResponse {
    set_todos_done(state, true).await
}

async fn mark_all_undone(State(state): State<AppState>) -> impl IntoResponse {
    set_todos_done(state, false).await
}

async fn set_todos_done(state: AppState, done: bool) -> impl IntoResponse {
    let mut todos = state.todos.write().await;

    for todo in todos.iter_mut() {
        todo.done = done;
    }

    state.alert_stats();

    templating::TodosTemplate {
        todos: todos.to_vec(),
    }
}
