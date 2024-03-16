use axum::{extract::State, response::IntoResponse, routing::get, Form, Router};
use serde::Deserialize;

use crate::{
    state::{AppState, Todo},
    templating,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_todo).post(add_todo))
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
