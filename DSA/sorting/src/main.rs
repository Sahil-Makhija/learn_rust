use std::{println, vec};

pub mod bubble_sort;
pub mod selection_sort;
pub mod utils;

fn main() {
    println!("Hello, world!");

    let mut nums: Vec<usize> = vec![13, 46, 24, 52, 20, 9];

    // selection_sort::selection_sort(&mut nums);
    // println!("Sorted Vector (Selection Sort) : {:?}", nums);

    bubble_sort::bubble_sort(&mut nums);
    println!("Sorted Vector (Bubble Sort) : {:?}", nums);
}
