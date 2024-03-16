use askama::Template;

use crate::state::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

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
