# 📋 todo-cli

A fast and minimal todo list manager for the terminal, built with Rust.

## Features

- Add, list, complete and remove todos
- Completed todos are automatically moved to the bottom with strikethrough
- IDs stay sequential after every operation
- Bulk-remove all completed todos in one command
- Data persisted locally in a plain text file

## Installation

```bash
git clone https://github.com/youruser/todo-cli
cd todo-cli
cargo install --path .
```

## Usage

```bash
todo add "buy bread"       # Add a new todo
todo list                  # List all todos
todo done 2                # Mark todo #2 as done
todo remove 3              # Remove todo #3
todo remove done           # Remove all completed todos
```

### Example output

```
📋 TODO LIST

📌 Pending:
  [ ] 1 - buy bread
  [ ] 2 - read something
  [ ] 3 - play tetrio

✅ Done:
  [x] 4 - write readme
```

## Project Structure

```
src/
├── main.rs        # Entry point, command routing
├── cli.rs         # Clap CLI definitions
├── commands.rs    # Command logic (add, list, done, remove)
├── sorting.rs     # ID reordering logic
├── storage.rs     # Load/save todos from disk
└── todo.rs        # Todo struct and serialization
```

## CI

This project uses GitHub Actions to run on every push and pull request:

- `cargo fmt` — enforces consistent formatting
- `cargo clippy` — lints and catches common mistakes
- `cargo test` — runs the test suite

## Data Storage

Todos are stored in a local `todos.txt` file in the working directory, one per line in the format:

```
id|title|done
```

## License

MIT
