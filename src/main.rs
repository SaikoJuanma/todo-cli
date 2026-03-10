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
        Command::Remove { id } => commands::remove(id),
    }
}
