// use std::io;

fn multiply_by_2(n: isize) -> isize {
    n * 2
}

fn multiply(x: isize, y: isize) -> isize {
    x * y
}
fn main() {
    let x = 5;
    let y = {
        let x = 4;
        x + 1
    };

    println!("Value of X : {x}");
    println!("Value of Y : {y}");

    println!("2 times {x} is {}", multiply_by_2(x));
    println!("{} times {} is {}", x, y, multiply(x, y));

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    println!()
}
