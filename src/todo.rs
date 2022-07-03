use crate::Task;
use std::error::Error;
use std::fmt;

pub trait Todo {
    type Err: Error + Send + Sync + 'static;
    fn add(&mut self, task: Task) -> Result<(), Self::Err>;
    fn remove(&mut self, index: usize) -> Result<(), Self::Err>;
    fn list(&mut self) -> Result<Vec<Task>, Self::Err>;
}

#[derive(Debug)]
pub struct TodoError {
    pub message: String,
}

impl TodoError {
    pub fn new(s: &str) -> TodoError {
        TodoError {
            message: s.to_string(),
        }
    }
}

impl fmt::Display for TodoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.message)
    }
}

impl Error for TodoError {}
