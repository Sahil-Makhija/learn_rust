use crate::utils::swap;

// If a function doesn't need mutable access, don't give it
pub fn get_minimum_index(nums: &[usize], curr_idx: usize) -> usize {
    let mut min_idx = curr_idx;
    for i in (curr_idx + 1)..(nums.len()) {
        if nums[i] < nums[min_idx] {
            min_idx = i;
        }
    }
    min_idx
}

pub fn selection_sort(nums: &mut Vec<usize>) {
    for i in 0..(nums.len()) {
        let min_idx = get_minimum_index(nums, i);
        if min_idx != i {
            swap(nums, i, min_idx);
        }
    }
}
