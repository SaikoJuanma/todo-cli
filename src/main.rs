mod cli;
mod commands;
mod sorting;
mod storage;
mod todo;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { title } => commands::add(title),
        Command::List => commands::list(),
        Command::Done { id } => commands::done(id),
        Command::Remove { target } => match target.as_str() {
            "done" => commands::remove_done(),
            _ => match target.parse::<u32>() {
                Ok(n) => commands::remove(n),
                Err(_) => println!("❌ Invalid argument: '{target}'. Use an id or 'done'"),
            },
        },
    }
}
