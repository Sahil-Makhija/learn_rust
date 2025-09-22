use std::io;

fn main() {
    println!("Please input a non-negative integer to calculate its factorial:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid non-negative integer.");
            return;
        }
    };

    if number == 0 {
        println!("The factorial of 0 is: 1");
        return;
    }

    let mut factorial: u32 = 1;
    for i in 1..=number {
        factorial *= i;
    }

    println!("The factorial of {} is: {}", number, factorial);
}
