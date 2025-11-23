struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut start = 0;
        let mut end = nums.len() - 1;

        while start <= end {
            let mid = start + (end - start) / 2;
            if nums[mid] > target {
                if mid == 0 {
                    break;
                }
                end = mid - 1;
            } else if nums[mid] < target {
                start = mid + 1;
            } else {
                return mid as i32;
            }
        }
        -1
    }

    // pub fn find_peak_element(nums: Vec<i32>) -> i32 {}

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut start = 0;
        let mut end = nums.len() - 1;

        while start <= end {
            let mid = start + (end - start) / 2;
            if (nums[mid] == target) {
                return mid as i32;
            } else if nums[mid] > target {
                if nums[mid - 1] < target {
                    return mid as i32;
                }
            }
        }

        return -1;
    }
}

fn main() {
    println!("Hello, world!");
}
