pub fn solve(arr: &Vec<isize>, n: isize) -> isize {
    let len = arr.len();
    if len < 1 {
        return -1;
    }
    let mut idx = 0;
    for i in arr {
        if *i == n {
            return idx;
        }
        idx += 1
    }
    -1
}
