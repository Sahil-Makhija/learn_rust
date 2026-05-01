use std::{io, process};

fn welcome() {
    println!("=====================================");
    println!("          User Input Looper          ");
    println!("=====================================");
}

#[derive(Debug)]
enum Command {
    Add,
    Subtract,
    Quit,
    None,
}

impl Command {
    fn handle_invalid_cmd(&self) {
        println!("Invalid Command, Try again!");
    }

    fn add(&self) {
        let mut n1 = String::new();
        io::stdin().read_line(&mut n1).expect("failed to read.");
        let n1 :u32 = match n1.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("failed to parse.");
            }
        }
        let mut n2 = String::new();
        io::stdin().read_line(&mut n2).expect("failed to read.");
        let n2 :u32 = match n2.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("failed to parse.");
            }
        }
        n1 + n2
    }

    fn subtract(&self) {
        println!("Subtract function here.");
    }

    fn quit(&self) {
        println!("Bye Bye!");
        process::exit(0x0100);
    }
}

fn take_user_input() -> Command {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!.");

    let args: Vec<&str> = input.trim().split(" ").collect();

    match args[0] {
        "/add" => Command::Add,
        "/subtract" => Command::Subtract,
        "/quit" => Command::Quit,
        _ => Command::None,
    }
}

fn main() {
    welcome();
    loop {
        let cmd = take_user_input();
        match cmd {
            Command::None => cmd.handle_invalid_cmd(),
            Command::Add => cmd.add(),
            Command::Subtract => cmd.subtract(),
            Command::Quit => cmd.quit(),
        };
    }
}
