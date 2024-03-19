use axum::response::sse::Event;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use tracing::error;

pub type Todos = Arc<RwLock<Vec<Todo>>>;
pub type IdCounter = Arc<RwLock<usize>>;

#[derive(Clone)]
pub struct Todo {
    pub idx: usize,
    pub done: bool,
    pub description: String,
}

#[derive(Clone)]
pub struct Stats {
    pub total: usize,
    pub done: usize,
    pub current_idx: usize,
}

#[derive(Clone)]
pub struct AppState {
    pub todos: Todos,
    pub id_counter: IdCounter,
    pub stats_tx: mpsc::UnboundedSender<()>,
    pub sse_tx: broadcast::Sender<Event>,
    pub trust_user_input: bool,
}

impl AppState {
    pub fn new(stats_tx: mpsc::UnboundedSender<()>, sse_tx: broadcast::Sender<Event>) -> Self {
        Self {
            todos: Default::default(),
            id_counter: Default::default(),
            stats_tx,
            sse_tx,
            trust_user_input: std::env::var("HTMX_TODO_TRUST_USER_INPUT")
                .map(|var| var == "true")
                .unwrap_or(false),
        }
    }

    pub fn alert_stats(&self) {
        if let Err(_) = self.stats_tx.send(()) {
            error!("failed to alert stats")
        }
    }

    pub async fn get_stats(&self) -> Stats {
        let todos = self.todos.read().await;

        Stats {
            total: todos.len(),
            done: todos.iter().filter(|todo| todo.done).count(),
            current_idx: *self.id_counter.read().await + 1,
        }
    }
}
