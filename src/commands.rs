use crate::sorting::reorder;
use crate::storage::{load_todos, save_todos};
use crate::todo::Todo;

pub fn add(title: String) {
    let mut todos = load_todos();
    // Push before done items, reorder will fix ids
    let insert_pos = todos.iter().position(|t| t.done).unwrap_or(todos.len());
    todos.insert(
        insert_pos,
        Todo {
            id: 0,
            title,
            done: false,
        },
    );
    let todos = reorder(todos);
    save_todos(&todos);
    println!(
        "✅ Added todo #{}",
        todos.iter().filter(|t| !t.done).count()
    );
}

pub fn list() {
    let todos = load_todos();
    if todos.is_empty() {
        println!("🎉 Nothing left to do!");
        return;
    }

    let pending: Vec<_> = todos.iter().filter(|t| !t.done).collect();
    let done: Vec<_> = todos.iter().filter(|t| t.done).collect();

    println!("\n📋 TODO LIST\n");

    if !pending.is_empty() {
        println!("📌 Pending:");
        for t in &pending {
            println!("  [ ] {} - {}", t.id, t.title);
        }
    }

    if !done.is_empty() {
        println!("\n✅ Done:");
        for t in &done {
            println!("  \x1b[9m[x] {} - {}\x1b[0m", t.id, t.title);
        }
    }

    println!();
}

pub fn done(id: u32) {
    let mut todos = load_todos();
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        let mut todo = todos.remove(pos);
        todo.done = true;
        todos.push(todo);
        let todos = reorder(todos);
        save_todos(&todos);
        println!("✅ Marked #{id} as done");
    } else {
        println!("❌ Todo #{id} not found");
    }
}

pub fn remove(id: u32) {
    let mut todos = load_todos();
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < before {
        let todos = reorder(todos);
        save_todos(&todos);
        println!("🗑️  Removed #{id}");
    } else {
        println!("❌ Todo #{id} not found");
    }
}

pub fn remove_done() {
    let mut todos = load_todos();
    let before = todos.len();
    todos.retain(|t| !t.done);
    let removed = before - todos.len();
    if removed > 0 {
        let todos = reorder(todos);
        save_todos(&todos);
        println!(
            "🗑️  Removed {removed} completed todo{}",
            if removed == 1 { "" } else { "s" }
        );
    } else {
        println!("✅ No completed todos to remove");
    }
}
