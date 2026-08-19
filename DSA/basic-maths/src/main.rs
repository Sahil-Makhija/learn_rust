use std::println;

fn count_digits(n: f32) -> u32 {
    (n.log10() as u32) + 1
}

fn reverse_number(mut n: usize) -> usize {
    let mut reversed = 0;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n = n / 10;
    }
    reversed
}

fn check_for_palindrome(n: usize) -> bool {
    reverse_number(n) == n
}

// Euclidean Algorithm
fn get_hcf(n1: usize, n2: usize) -> usize {
    if n2 == 0 {
        return n1;
    }
    get_hcf(n2, n1 % n2)
}

fn get_armstrong(mut n: u32) -> u32 {
    let digits = count_digits(n as f32);
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(digits);
        n /= 10;
    }
    sum
}

fn get_divisors(n: u32) -> Vec<u32> {
    let sqr_n = ((n as f32).sqrt()) as u32;
    let mut divisors: Vec<u32> = vec![1];
    for i in 2..(sqr_n + 1) {
        if (n % i) == 0 {
            divisors.push(i);
            if (n / i) != (i) {
                divisors.push(n / i);
            }
        }
    }
    divisors.push(n);
    divisors
}

fn check_for_prime(n: u32) -> bool {
    get_divisors(n).len() == 2
}

fn main() {
    println!("Hello, world!");

    let n = 5321142.0;
    println!("The number {n} has {} digit(s).", count_digits(n));

    let n = 1234532131;
    println!("The reverse of {n} is {}.", reverse_number(n));

    let n = 1331;
    println!("Palindrome? : {}", check_for_palindrome(n));

    let n1 = 444;
    let n2 = 666;
    println!("{}", get_hcf(n1, n2));

    let n: u32 = 37;
    println!("Number is an armstrong number : {}", n == get_armstrong(n));

    let n: u32 = 31;
    println!("The divisors of {n} are {:?}.", get_divisors(n));

    let n = 4;
    println!("Is {} a Prime number ? : {}", n, check_for_prime(n));
}
