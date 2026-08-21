use std::{println, vec};

pub mod bubble_sort;
pub mod insertion_sort;
pub mod leetcode;
pub mod selection_sort;
pub mod utils;

fn main() {
    println!("Hello, world!");

    // let mut nums: Vec<usize> = vec![5, 4, 4, 1, 1];

    // selection_sort::selection_sort(&mut nums);
    // println!("Sorted Vector (Selection Sort) : {:?}", nums);

    // bubble_sort::bubble_sort(&mut nums);
    // println!("Sorted Vector (Bubble Sort) : {:?}", nums);

    // insertion_sort::insertion_sort(&mut nums);
    // println!("Sorted Vector (Insertion Sort) : {:?}", nums);

    let mut nums1: Vec<i32> = vec![1, 3];
    let mut nums2: Vec<i32> = vec![2, 4];
    let median = leetcode::find_median_sorted_arrays(&mut nums1, &mut nums2);
    println!("Nums1={:?}", nums1);
    println!("Nums2={:?}", nums2);
    println!("Median={:?}", median);
}
