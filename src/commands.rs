use crate::storage::{load_todos, save_todos};
use crate::todo::Todo;

pub fn add(title: String) {
    let mut todos = load_todos();
    let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    todos.push(Todo {
        id,
        title,
        done: false,
    });
    save_todos(&todos);
    println!("Added todo #{id}");
}

pub fn list() {
    let todos = load_todos();
    if todos.is_empty() {
        println!("Nothing left to do!");
        return;
    }
    for t in &todos {
        let mark = if t.done { "[x]" } else { "[ ]" };
        println!("{} {} - {}", mark, t.id, t.title);
    }
}

pub fn done(id: u32) {
    let mut todos = load_todos();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = true;
        println!("Marked #{id} as done");
    } else {
        println!("Todo #{id} not found");
    }
    save_todos(&todos);
}

pub fn remove(id: u32) {
    let mut todos = load_todos();
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < before {
        println!("Removed #{id}");
    } else {
        println!("Todo #{id} not found");
    }
    save_todos(&todos);
}
