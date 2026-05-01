// Assumption : arr contains only +ve integers

use std::isize;

pub fn solve(arr: &Vec<isize>) -> (isize, isize) {
    if arr.len() < 2 {
        return (-1, -1);
    }
    let mut largest_elem = isize::MIN;
    let mut second_largest = isize::MIN;

    let mut smallest_elem = isize::MAX;
    let mut second_smallest = isize::MAX;

    for i in arr {
        if *i > largest_elem {
            second_largest = largest_elem;
            largest_elem = *i;
        } else if *i > second_largest && *i < largest_elem {
            second_largest = *i;
        }

        if *i < smallest_elem {
            second_smallest = smallest_elem;
            smallest_elem = *i;
        } else if *i < second_smallest && *i > smallest_elem {
            second_smallest = *i;
        }
    }

    (second_largest, second_smallest)
}
