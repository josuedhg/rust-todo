use std::fs::OpenOptions;
use todo::Task;
use todo::Todo;
use todo::TodoTxt;

fn main() {
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.txt")
    {
        Ok(file) => file,
        Err(e) => panic!("Could not open file: {}", e),
    };
    let mut todo = TodoTxt::new(Box::new(file));
    let task = Task::new(String::from("Learn Rust"), String::from("Learn"), 'A');

    match todo.add(task) {
        Ok(_) => (),
        Err(e) => panic!("Could not add task: {}", e),
    };

    let list = match todo.list() {
        Ok(list) => list,
        Err(e) => panic!("Could not list tasks: {}", e),
    };
    for task in list {
        println!("{}", task.name);
    }
}
