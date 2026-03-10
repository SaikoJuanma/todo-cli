# todo-cli

A simple command line **Todo manager written in Rust**.

This project is a lightweight example of a CLI application using:

-   Rust
-   clap for argument parsing
-   simple file-based storage
-   CI workflows for linting, testing, security and cross-compilation

The goal of this project is to demonstrate a clean and minimal Rust CLI
architecture.

------------------------------------------------------------------------

# Features

-   Add todos
-   List todos
-   Mark todos as done
-   Remove todos
-   File-based persistence (todos.txt)

------------------------------------------------------------------------

# Installation

## 1. Clone the repository

git clone https://github.com/your-user/todo-cli.git cd todo-cli

## 2. Build

cargo build --release

Binary will be located at:

target/release/todo

Optional installation:

cargo install --path .

------------------------------------------------------------------------

# Usage

## Add a todo

todo add "Buy milk"

Example output:

Added todo #1

------------------------------------------------------------------------

## List todos

todo list

Example output:

\[ \] 1 - Buy milk \[x\] 2 - Finish report

Legend:

\[ \] -\> not completed\
\[x\] -\> completed

------------------------------------------------------------------------

## Mark a todo as done

todo done 1

Output:

Marked #1 as done

------------------------------------------------------------------------

## Remove a todo

todo remove 1

Output:

Removed #1

------------------------------------------------------------------------

# Storage

Todos are stored in:

todos.txt

Format:

id\|title\|done

Example:

1\|Buy milk\|false 2\|Finish report\|true

------------------------------------------------------------------------

# Project Structure

src/ ├── main.rs \# entry point ├── cli.rs \# CLI definitions ├──
commands.rs \# command implementations ├── storage.rs \# file
persistence └── todo.rs \# Todo structure

------------------------------------------------------------------------

# Development

Format:

cargo fmt

Lint:

cargo clippy

Test:

cargo test

------------------------------------------------------------------------

# CI

The project includes CI workflows for:

-   cargo check
-   cargo fmt
-   cargo clippy
-   cargo test
-   cargo audit
-   cargo deny
-   cross compilation
-   .deb packaging smoke tests

------------------------------------------------------------------------

# Future Improvements

-   JSON storage
-   task editing
-   priorities
-   due dates
-   colored output
-   TUI interface
-   SQLite backend
-   sync support

------------------------------------------------------------------------

# License

MIT
