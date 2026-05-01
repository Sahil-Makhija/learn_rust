use std::{io, isize, process};

enum Command {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    println!(
        "Terminal Calculator for 2 numbers.\nAvailable Commands:\n\t- /a (Add)\n\t- /s (Subtract)\n\t- /m (Multiply)\n\t- /d (Divide)\n\t- /x (exit)"
    );

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read.");

        let cmd: Command = match input.trim() {
            "/a" => Command::Add,
            "/s" => Command::Subtract,
            "/m" => Command::Multiply,
            "/d" => Command::Divide,
            "/x" => process::exit(0),
            _ => {
                println!("Invalid command");
                continue;
            }
        };

        if let Ok((n1, n2)) = parse_numbers() {
            match cmd {
                Command::Add => println!("Ans = {}", n1 + n2),
                Command::Subtract => println!("Ans = {}", n1 - n2),
                Command::Multiply => println!("Ans = {}", n1 * n2),
                Command::Divide => println!("Ans = {}", n1 / n2),
            };
        }
    }
}

fn parse_numbers() -> Result<(isize, isize), ()> {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).map_err(|_| ())?;

    let n1: isize = input.trim().parse().map_err(|_| ())?;

    input.clear();
    println!("Enter second number:");
    io::stdin().read_line(&mut input).map_err(|_| ())?;

    let n2: isize = input.trim().parse().map_err(|_| ())?;

    Ok((n1, n2))
}
