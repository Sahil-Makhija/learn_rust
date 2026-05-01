pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut min_speed = 1;
    let mut max_speed = 0;
    for &i in &piles {
        max_speed += i;
    }

    while min_speed < max_speed {
        let avg_speed = (max_speed + min_speed) / 2;
        if is_speed_valid(&piles, avg_speed, h) {
            max_speed = avg_speed;
        } else {
            min_speed = avg_speed + 1;
        }
    }

    min_speed
}

pub fn is_speed_valid(piles: &Vec<i32>, speed: i32, h: i32) -> bool {
    let mut time_taken = 0;
    for &i in piles {
        time_taken += (i + speed - 1) / speed;
        if time_taken > h {
            return false;
        }
    }
    true
}
