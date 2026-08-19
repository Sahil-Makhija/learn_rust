use crate::utils::swap;

pub fn bubble_sort(nums: &mut Vec<usize>) {
    let mut end = nums.len();
    while end > 1 {
        let mut swapped = false;
        for i in 1..(end) {
            if nums[i] < nums[i - 1] {
                swap(nums, i, i - 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        end -= 1;
    }
}
