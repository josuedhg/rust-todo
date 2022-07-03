use todo::Task;
use std::fs::OpenOptions;
use todo::TodoTxt;
use todo::Todo;

fn main() {
    let file = OpenOptions::new().read(true).write(true).create(true).open("todo.txt").unwrap();
    let mut todo = TodoTxt::new(Box::new(file));
    let task = Task::new(String::from("Learn Rust"), String::from("Learn"), 'A');
    todo.add(task).unwrap();
    for task in todo.list().unwrap() {
        println!("{}", task.name);
    }
}
