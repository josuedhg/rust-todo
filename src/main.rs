use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use todo::Task;
use todo::Todo;
use todo::TodoTxt;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { task: Vec<String> },
    List,
}

impl Command {
    fn run(&self, todo: &mut TodoTxt) {
        match self {
            Command::List => {
                let list = match todo.list() {
                    Ok(list) => list,
                    Err(e) => panic!("Could not list tasks: {}", e),
                };
                for task in list {
                    println!("{}", task.name);
                }
            }
            Command::Add { task } => {
                let newtask = Task::new(task.join(" "), String::from(""), 'A');

                match todo.add(newtask) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not add task: {}", e),
                };
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

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

    match args.command {
        Some(command) => command.run(&mut todo),
        None => println!("No command provided"),
    };
}
