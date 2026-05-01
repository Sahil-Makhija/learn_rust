use std::isize;

pub fn solve(arr: &Vec<isize>) -> bool {
    let mut curr_val = isize::MIN;
    for i in arr {
        if *i < curr_val {
            return false;
        }
        curr_val = *i;
    }
    true
}
