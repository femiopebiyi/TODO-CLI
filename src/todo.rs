use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(id: usize, task: String, due_in: u32) -> Self {
        Self {
            id,
            task,
            completed: false,
            created_at: Utc::now(),
            due_date: if due_in > 0 {
                Some(Utc::now() + chrono::Duration::days(due_in as i64))
            } else {
                None
            },
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
                "[x] {}. {} (created: {}), due date: {})",
                self.id,
                self.task,
                self.created_at.format("%Y-%m-%d %H:%M:%S"),
                self.due_date.as_ref().map_or_else(
                    || "None".into(),
                    |date| date.format("%Y-%m-%d %H:%M:%S").to_string()
                )
            )
        } else {
            format!(
                "[ ] {}. {} (created: {}), due date: {})",
                self.id,
                self.task,
                self.created_at.format("%Y-%m-%d %H:%M:%S"),
                self.due_date.as_ref().map_or_else(
                    || "None".into(),
                    |date| date.format("%Y-%m-%d %H:%M:%S").to_string()
                )
            )
        }
    }
}
