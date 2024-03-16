use askama::Template;

use crate::state::{Stats, Todo};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub stats: Stats,
}

#[derive(Template, Default)]
#[template(path = "todos.html")]
pub struct TodosTemplate {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo.html")]
pub struct TodoTemplate {
    pub todo: Todo,
}

#[derive(Template)]
#[template(path = "add-todo.html")]
pub struct AddTodoTemplate {
    pub todo: Todo,
}
