mod cli;
mod storage;
mod todo;

use std::process;

use clap::Parser;
use cli::{Cli, Commands};
use storage::Storage;

use colored::Colorize;

fn main() {
    let cli = Cli::parse();
    let mut storage = Storage::new("todos.json");

    // TODO: Load existing todos from file
    match storage.load() {
        Ok(_) => (),
        Err(error) => {
            println!(" this {}", error);
            process::exit(1);
        }
    }
    // HINT: storage.load()

    match cli.command {
        Commands::Add { task } => {
            // TODO: Create a new Todo

            // TODO: Add it to storage
            storage.add(task.clone().trim().to_string());
            // TODO: Save to file
            match storage.save() {
                Ok(_) => (),
                Err(error) => println!("{}", error),
            }
            // TODO: Print success message
            println!("✅ Added task: {}", task);
        }

        Commands::List => {
            // TODO: Get all todos from storage

            // TODO: If empty, print "No tasks found"
            let todos = storage.get_all();
            if todos.is_empty() {
                println!("No tasks found");
                return;
            }

            todos.iter().for_each(|todo| {
                let display = todo.display();
                if todo.is_completed() {
                    println!("{}", display.green());
                } else {
                    println!("{}", display.yellow());
                }
            });
            // TODO: Otherwise, print each todo with index
            // HINT: Use colored crate for pretty output
            // BONUS: Show different colors for completed vs pending
        }

        Commands::Complete { id } => {
            match storage.complete(id) {
                Ok(_) => (),
                Err(error) => {
                    println!("{}", error);
                    process::exit(0);
                }
            }

            match storage.save() {
                Ok(_) => (),
                Err(error) => {
                    println!("Error saving todos: {}", error);
                    process::exit(1);
                }
            }
            // TODO: Print success message
            // HINT: Handle case where id doesn't exist
            println!("✅ Completed task #{}", id);
        }

        Commands::Remove { id } => {
            // TODO: Remove todo by id
            // TODO: Save to file
            // TODO: Print success message
            // HINT: Handle case where id doesn't exist
            match storage.remove(id) {
                Ok(_) => (),
                Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                }
            }

            match storage.save() {
                Ok(_) => (),
                Err(error) => {
                    println!("Error saving todos: {}", error);
                    process::exit(1);
                }
            }
            println!("🗑️  Removed task #{}", id);
        }

        Commands::Clear => {
            // TODO: Remove all completed tasks
            let cleared = storage.clear_completed();
            // TODO: Save to file
            match storage.save() {
                Ok(_) => (),
                Err(error) => {
                    println!("Error saving todos: {}", error);
                    process::exit(1);
                }
            }
            // TODO: Print how many tasks were cleared
            println!("🧹 Cleared {} completed tasks", cleared);
        }
    }

    // TODO: Handle any errors that occurred
}
