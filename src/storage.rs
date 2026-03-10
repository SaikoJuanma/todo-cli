use crate::todo::Todo;
use std::fs;

fn get_file_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".local/share/todo-cli/todos.txt");
    path
}

pub fn load_todos() -> Vec<Todo> {
    let path = get_file_path();
    let content = fs::read_to_string(&path).unwrap_or_default();
    content.lines().filter_map(Todo::from_line).collect()
}

pub fn save_todos(todos: &[Todo]) {
    let path = get_file_path();
    fs::create_dir_all(path.parent().unwrap()).expect("Could not create data directory");
    let content = todos
        .iter()
        .map(|t| t.to_line())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, content).expect("Could not save todos");
}
