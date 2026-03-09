use std::{error::Error, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::todo::Todo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(skip)]
    file_path: String,
    pub todos: Vec<Todo>,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            todos: Vec::new(),
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        if Path::new(&self.file_path).try_exists()? {
            let contents = fs::read_to_string(&self.file_path)?;

            self.todos = serde_json::from_str(&contents)?;
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self.todos)?;
        fs::write(&self.file_path, serialized)?;
        Ok(())
    }

    pub fn add(&mut self, task: String, due_in: u32) {
        let id = self.todos.len() + 1;
        let new_todo = Todo::new(id, task, due_in);

        self.todos.push(new_todo);
    }

    pub fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn complete(&mut self, id: usize) -> Result<(), String> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            if !todo.is_completed() {
                todo.mark_completed();
            } else {
                return Err(format!(
                    "Todo with id {}: ({}) is already completed",
                    id, todo.task
                ));
            }
            Ok(())
        } else {
            Err(format!("Todo with id {} not found", id))
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);
            Ok(())
        } else {
            Err(format!("Todo with id {} not found", id))
        }
    }

    pub fn clear_completed(&mut self) -> usize {
        let before_length = self.todos.len();
        self.todos.retain(|t| !t.is_completed());
        before_length - self.todos.len()
    }

    pub fn edit(&mut self, new_task: String, id: usize) -> Result<(), String> {
        if let Some(todo) = self
            .todos
            .iter_mut()
            .find(|t| t.id == id && !t.is_completed())
        {
            todo.task = new_task;
            Ok(())
        } else {
            Err(format!("Todo with id {} not found", id))
        }
    }
}
