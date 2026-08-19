use std::println;

fn print_n_times(content: &str, n: u32) {
    if n != 0 {
        println!("{content}");
        print_n_times(content, n - 1);
    }
}

fn one_to_n(n: u32, c: u32) {
    if c > n {
        return;
    }
    println!("{c}");
    one_to_n(n, c + 1);
}

fn n_to_one(n: u32) {
    if n == 0 {
        return;
    }
    println!("{n}");
    n_to_one(n - 1);
}

fn sum_of_n_natural_numbers(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    n + sum_of_n_natural_numbers(n - 1)
}

fn main() {
    // print_n_times("Hello, world!", 5);
    // n_to_one(6);
    // one_to_n(6, 1);
    println!(
        "Sum of first {} natural numbers is : {}",
        5,
        sum_of_n_natural_numbers(5)
    );
}
