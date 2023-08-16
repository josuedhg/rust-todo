use crate::{Task, Todo, TodoError};
use std::io::{Read, Write, Seek};
use std::str::FromStr;
use std::string::ToString;

pub trait TodotxtIO: Read + Write + Seek {}
impl TodotxtIO for std::fs::File {}

impl FromStr for Task {
    type Err = TodoError;

    fn from_str(s: &str) -> Result<Task, TodoError> {
        if s.is_empty() {
            return Err(TodoError::new("Cannot parse empty string"));
        }
        let completed = s.starts_with('x');
        let x: &[_] = &['x', ' ', '(' , ')'];
        let priority = s.trim_matches(x).chars().next().ok_or(TodoError::new("Cannot parse task priority"))?;
        let name = s.trim_start_matches(x).trim_start_matches(char::is_alphabetic).trim_start_matches(x);

        let mut project = String::new();
        for word in name.split_whitespace() {
            if word.starts_with('+') {
                project = word.to_string().trim_start_matches('+').to_string();
                break;
            }
        }

        let mut task = Task::new(name.to_string(), project, priority);
        if completed {
            task.complete();
        }
        Ok(task)
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.is_complete() {
            s.push('x');
            s.push(' ');
        }
        s.push('(');
        s.push(self.get_priority());
        s.push(')');
        s.push(' ');
        s.push_str(&self.name);
        if !self.project.is_empty() && !self.name.contains(format!("+{}", self.project).as_str()) {
            s.push(' ');
            s.push('+');
            s.push_str(&self.project);
        }
        s
    }

}

pub struct TodoTxt {
    io: Box<dyn TodotxtIO>,
    tasks: Vec<Task>,
}

impl TodoTxt {
    pub fn new(io: Box<dyn TodotxtIO>) -> TodoTxt {
        TodoTxt {
            io,
            tasks: Vec::new(),
        }
    }

    pub fn load(&mut self) -> Result<(), TodoError> {
        let mut content = String::new();
        self.io.read_to_string(&mut content).map_err(|e| TodoError::new(&format!("{}", e)))?;

        if content.is_empty() {
            return Ok(());
        }

        for line in content.lines() {
            if line.starts_with('#') {
                continue;
            }
            let task = Task::from_str(line)?;
            self.tasks.push(task);
        }
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), TodoError> {
        let mut content = String::new();
        for task in &self.tasks {
            content.push_str(&format!("{}\n", task.to_string()));
        }
        self.io.rewind().map_err(|e| TodoError::new(&format!("{}", e)))?;
        self.io.write_all(content.as_bytes()).map_err(|e| TodoError::new(&format!("{}", e)))?;
        self.io.flush().map_err(|e| TodoError::new(&format!("{}", e)))?;
        Ok(())
    }
}

impl Todo for TodoTxt {
    type Err = TodoError;
    fn add(&mut self, task: Task) -> Result<(), TodoError> {
        self.load()?;
        self.tasks.push(task);
        self.save()?;
        Ok(())
    }
    fn remove(&mut self, index: usize) -> Result<(), TodoError> {
        self.load()?;
        self.tasks.remove(index);
        self.save()?;
        Ok(())
    }
    fn list(&mut self) -> Result<Vec<Task>, TodoError> {
        self.load()?;
        Ok(self.tasks.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockIO {
        content: String,
        read: bool,
    }

    impl MockIO {
        fn from_string(content: String) -> MockIO {
            MockIO {
                content,
                read: false,
            }
        }

        fn new() -> MockIO {
            MockIO {
                content: String::new(),
                read: false,
            }
        }
    }

    impl Read for MockIO {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.content.as_bytes().read(buf)?;
            if !self.read {
                self.read = true;
                Ok(self.content.len())
            } else {
                Ok(0)
            }
        }
    }

    impl Write for MockIO {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Seek for MockIO {
        #[allow(unused_variables)]
        fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
            Ok(0)
        }
    }

    impl TodotxtIO for MockIO {}

    #[test]
    fn test_add() {
        let mut todo = TodoTxt::new(Box::new(MockIO::new()));
        let task = Task::new("test".to_string(), "project".to_string(), 'A');
        if let Err(e) = todo.add(task) {
            panic!("{}", e);
        }

        if let Ok(list) = todo.list() {
            assert_eq!(list.len(), 1);
        }
    }

    #[test]
    fn test_remove() {
        let mut todo = TodoTxt::new(Box::new(MockIO::new()));
        let task = Task::new("test".to_string(), "project".to_string(), 'A');
        if let Err(e) = todo.add(task) {
            panic!("{}", e);
        }

        if let Err(e) = todo.remove(0) {
            panic!("{}", e);
        }

        if let Ok(list) = todo.list() {
            assert_eq!(list.len(), 0);
        }
    }

    #[test]
    fn test_list() {
        let mut todo = TodoTxt::new(Box::new(MockIO::from_string("(A) test +project".to_string())));
        if let Ok(list) = todo.list() {
            assert_eq!(list.len(), 1);
        }
    }

    #[test]
    fn test_list_empty() {
        let mut todo = TodoTxt::new(Box::new(MockIO::new()));
        if let Ok(list) = todo.list() {
            assert_eq!(list.len(), 0);
        }
    }

    #[test]
    fn test_task_from_str_invalid_format() {
        let task = Task::from_str("");
        assert!(task.is_err());
        if let Some(err) = task.err() {
            assert_eq!(format!("{}", err), "Cannot parse empty string");
        }
    }

    #[test]
    fn test_task_from_str_no_project() {
        match Task::from_str("(A) Learn Rust") {
            Ok(task) => {
                assert_eq!(task.name, "Learn Rust");
                assert_eq!(task.project, "");
                assert_eq!(task.get_priority(), 'A');
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_task_from_str() {
        match Task::from_str("x (A) Learn Rust +project") {
            Ok(task) => {
                assert_eq!(task.name, "Learn Rust +project");
                assert_eq!(task.project, "project");
                assert_eq!(task.is_complete(), true);
                assert_eq!(task.get_priority(), 'A');
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_task_from_str_completed() {
        match Task::from_str("x (A) Learn Rust +project") {
            Ok(task) => assert_eq!(task.is_complete(), true),
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_task_to_str_no_project() {
        let task = Task::new(String::from("Learn Rust"), String::from(""), 'A');
        assert_eq!(task.to_string(), "(A) Learn Rust");
    }

    #[test]
    fn test_task_to_str() {
        let task = Task::new(String::from("Learn Rust"), String::from("project"), 'A');
        assert_eq!(task.to_string(), "(A) Learn Rust +project");
    }

    #[test]
    fn test_task_to_str_completed() {
        let mut task = Task::new(String::from("Learn Rust"), String::from("project"), 'A');
        task.complete();
        assert_eq!(task.to_string(), "x (A) Learn Rust +project");
    }
}
