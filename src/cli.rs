use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple todo list manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task description
        task: String,
    },

    /// List all tasks
    List,

    /// Mark a task as complete
    Complete {
        /// The task ID to complete
        id: usize,
    },

    /// Remove a task
    Remove {
        /// The task ID to remove
        id: usize,
    },

    /// Clear all completed tasks
    Clear,
}
