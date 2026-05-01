pub fn search_insert(nums: &Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len < 1 {
        return 0;
    }

    if target < nums[0] {
        return 0;
    } else if target > nums[len - 1] {
        return len as i32;
    }

    let mut start = 0;
    let mut end = len - 1;

    while start < end {
        let mid = (start + end) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            if nums[mid - 1] < target {
                return mid as i32;
            } else {
                end = mid - 1;
            }
        } else {
            if nums[mid + 1] > target {
                return mid as i32;
            } else {
                start = mid + 1;
            }
        }
    }

    len as i32
}
