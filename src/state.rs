use std::sync::Arc;
use tokio::sync::RwLock;

pub type Todos = Arc<RwLock<Vec<Todo>>>;
pub type IdCounter = Arc<RwLock<usize>>;

#[derive(Clone)]
pub struct Todo {
    pub idx: usize,
    pub done: bool,
    pub description: String,
}

#[derive(Clone)]
pub struct AppState {
    pub todos: Todos,
    pub id_counter: IdCounter,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Default::default(),
            id_counter: Default::default(),
        }
    }
}
