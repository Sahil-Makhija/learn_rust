pub fn insertion_sort(nums: &mut Vec<usize>) {
    let mut start = 1;
    let end = nums.len();

    while start < end {
        let mut key = start;
        for i in (0..(start)).rev() {
            if nums[i] <= nums[key] {
                break;
            } else {
                nums.swap(i, key);
                key = i;
            }
        }
        start += 1;
    }
}
