pub fn swap(nums: &mut Vec<usize>, i1: usize, i2: usize) {
    let tmp = nums[i1];
    nums[i1] = nums[i2];
    nums[i2] = tmp;
}
