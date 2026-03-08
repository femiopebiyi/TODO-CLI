use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(id: usize, task: String) -> Self {
        Self {
            id,
            task,
            completed: false,
            created_at: Utc::now(),
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn display(&self) -> String {
        if self.is_completed() {
            format!(
                "[x] {}. {} (created: {})",
                self.id,
                self.task,
                self.created_at.format("%Y-%m-%d %H:%M:%S")
            )
        } else {
            format!(
                "[ ] {}. {} (created: {})",
                self.id,
                self.task,
                self.created_at.format("%Y-%m-%d %H:%M:%S")
            )
        }
    }
}
