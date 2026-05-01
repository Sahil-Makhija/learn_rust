use std::{io, process};

#[derive(Debug)]
enum Status {
    Completed,
    Pending,
    InProgress,
}

#[derive(Debug)]
struct Todo {
    task_name: String,
    status: Status,
}

enum Command {
    Create,
    List,
    Delete,
    Update,
    Exit,
    Invalid,
}

fn parse_command(input: &String) -> Command {
    let cmd = match input.trim() {
        "/create" => Command::Create,
        "/list" => Command::List,
        "/delete" => Command::Delete,
        "/update" => Command::Update,
        "/exit" => Command::Exit,
        _ => Command::Invalid,
    };
    cmd
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read.");
        let cmd = parse_command(&input);

        match cmd {
            Command::Create => {
                input.clear();
                println!("Enter task name:");
                io::stdin().read_line(&mut input).expect("failed to read.");
                let todo = Todo {
                    task_name: String::from(input.trim()),
                    status: Status::Pending,
                };
                println!("New todo created.\n {todo:?}");
                todos.push(todo);
            }
            Command::List => {
                for (idx, t) in todos.iter().enumerate() {
                    println!("{} | {t:?}", idx + 1);
                }
            }
            Command::Update => println!("Update a todo."),
            Command::Delete => {
                println!("Enter Todo no. to delete :");
                input.clear();
                io::stdin().read_line(&mut input).expect("failed to read.");
                let todo_idx: usize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("please enter a valid numerical index.");
                        continue;
                    }
                };
            }
            Command::Exit => process::exit(0),
            Command::Invalid => println!("Invalid command."),
        }
    }
}
