use std::io;

// fn get_sum(n: u32) -> u32 {
//     if n == 0 || n == 1 {
//         return n;
//     }

//     return get_sum(n - 1) + get_sum(n - 2);
// }

fn get_fibonacci_sum(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    return curr;
}

fn main() {
    println!("Please enter a non-negative number.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    let value: u32 = get_fibonacci_sum(input);
    println!("Value at given index is {}", value);
}
