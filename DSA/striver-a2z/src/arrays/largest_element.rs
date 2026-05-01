pub fn solve(arr: &Vec<isize>) -> isize {
    let mut largest_elem = isize::MIN;
    for i in arr {
        if i > &largest_elem {
            largest_elem = *i;
        }
    }
    largest_elem
}
