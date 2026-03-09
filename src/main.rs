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
        Commands::Add { task, due } => {
            // TODO: Create a new Todo

            // TODO: Add it to storage
            storage.add(task.clone().trim().to_string(), due.unwrap_or(0));
            // TODO: Save to file
            save_or_exit(&storage);
            // TODO: Print success message
            println!(
                "✅ Added task: {}  ({})",
                task,
                due.map_or_else(|| "no due date".into(), |d| format!("due in {} days", d))
            );
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

            save_or_exit(&storage);
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

            save_or_exit(&storage);
            println!("🗑️  Removed task #{}", id);
        }

        Commands::Clear => {
            // TODO: Remove all completed tasks
            let cleared = storage.clear_completed();
            // TODO: Save to file
            save_or_exit(&storage);
            // TODO: Print how many tasks were cleared
            println!("🧹 Cleared {} completed tasks", cleared);
        }

        Commands::Search { keyword } => {
            let keyword = keyword.trim().to_lowercase();
            let todos = storage.get_all();

            let results: Vec<_> = todos
                .iter()
                .filter(|todo| todo.task.to_lowercase().contains(&keyword))
                .collect();

            if results.is_empty() {
                println!("No tasks found matching '{}'", keyword);
            } else {
                println!("Found {} task(s) matching '{}':", results.len(), keyword);
                results.iter().for_each(|todo| {
                    let display = todo.display();
                    if todo.is_completed() {
                        println!("{}", display.green());
                    } else {
                        println!("{}", display.yellow());
                    }
                });
            }
        }
        Commands::Stats => {
            let todos = storage.get_all();
            let total = todos.len();
            let completed = todos.iter().filter(|t| t.is_completed()).count();
            let pending = total - completed;

            println!("📊 Task Statistics:");
            println!("Total tasks: {}", total);
            println!("Completed: {}", completed);
            println!("Pending: {}", pending);
            println!(
                "Completion rate: {:.2}%",
                if total > 0 {
                    (completed as f64 / total as f64) * 100.0
                } else {
                    0.0
                }
            );
        }
        Commands::Edit { id, new_task } => {
            match storage.edit(new_task.clone(), id) {
                Ok(_) => (),
                Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                }
            }

            save_or_exit(&storage);
            println!("✏️  Updated task #{} to '{}'", id, new_task);
        }
    }

    // TODO: Handle any errors that occurred
}

fn save_or_exit(storage: &Storage) {
    if let Err(error) = storage.save() {
        println!("Error saving todos: {}", error);
        process::exit(1);
    }
}

// Then use:
