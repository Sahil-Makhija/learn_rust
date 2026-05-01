pub fn min_days(bloom_day: &Vec<i32>, m: i32, k: i32) -> i32 {
    let (mut max_days, mut min_days) = get_max_min(bloom_day);

    if (bloom_day.len() as i64) < (m as i64 * k as i64) {
        return -1;
    }

    while min_days < max_days {
        let avg_days = (min_days + max_days) / 2;

        // Check how many bouquets can be created
        // compare to the required no. of bouquets
        // adjust min max days accordingly
        if get_max_bouquets(&bloom_day, k, avg_days) >= m {
            max_days = avg_days;
        } else {
            min_days = avg_days + 1;
        }
    }

    max_days
}

pub fn get_max_bouquets(bloom_day: &Vec<i32>, k: i32, d: i32) -> i32 {
    let mut bloomed = 0;
    let mut bouquets = 0;
    for i in bloom_day {
        if *i <= d {
            bloomed += 1;
        } else {
            bloomed = 0;
        }

        if bloomed == k {
            bouquets += 1;
            bloomed = 0;
        }
    }
    bouquets
}

pub fn get_max_min(arr: &Vec<i32>) -> (i32, i32) {
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    for i in arr {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }
    (max, min)
}
