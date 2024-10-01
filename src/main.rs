mod task;
mod commands;
mod task_manager;

use crate::commands::{Cli, Commands};
use clap::Parser;
use crate::task_manager::{add_task, list_tasks, complete_task, delete_task};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { description, category, priority } => {
            add_task(description, category, priority);
        },
        Commands::List { category } => {
            list_tasks(category);
        },
        Commands::Complete { id } => {
            complete_task(*id);
        },
        Commands::Delete { id } => {
            delete_task(*id);
        }
    }
}