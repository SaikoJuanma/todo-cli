use crate::todo::Todo;
use std::fs;

const FILE: &str = "todos.txt";

pub fn load_todos() -> Vec<Todo> {
    let content = fs::read_to_string(FILE).unwrap_or_default();
    content.lines().filter_map(Todo::from_line).collect()
}

pub fn save_todos(todos: &[Todo]) {
    let content = todos
        .iter()
        .map(|t| t.to_line())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(FILE, content).expect("Could not save todos");
}
