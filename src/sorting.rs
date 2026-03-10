use crate::todo::Todo;

pub fn reorder(todos: Vec<Todo>) -> Vec<Todo> {
    let (mut pending, mut done): (Vec<Todo>, Vec<Todo>) = todos.into_iter().partition(|t| !t.done);

    for (i, t) in pending.iter_mut().enumerate() {
        t.id = (i + 1) as u32;
    }

    let offset = pending.len() as u32;
    for (i, t) in done.iter_mut().enumerate() {
        t.id = offset + (i + 1) as u32;
    }

    pending.extend(done);
    pending
}
