fn print_square(n: u32) {
    for _i in 0..n {
        for _j in 0..n {
            print!("* ");
        }
        print!("\n");
    }
}

fn print_right_triangle(n: u32) {
    for i in 0..n {
        for _j in 0..(i + 1) {
            print!("* ");
        }
        print!("\n");
    }
}

fn print_numbered_triangle(n: u32) {
    for i in 0..n {
        for j in 1..(i + 2) {
            print!("{}", j);
        }
        print!("\n");
    }
}

fn pattern_fourth(n: u32) {
    for i in 1..(n + 1) {
        for _j in 0..i {
            print!("{}", i);
        }
        print!("\n");
    }
}

fn pattern_fifth(n: u32) {
    for i in 0..n {
        for _j in 0..(n - i) {
            print!("*");
        }
        print!("\n");
    }
}

fn pattern_sixth(n: u32) {
    for i in 0..n {
        for j in 0..(n - i) {
            print!("{}", j + 1)
        }
        print!("\n");
    }
}

fn pattern_seventh(n: u32) {
    for i in 0..n {
        // print spaces
        for _j in 0..(n - i - 1) {
            print!(" ");
        }
        // print stars
        for _j in 0..((2 * i) + 1) {
            print!("*");
        }
        print!("\n");
    }
}

fn pattern_eighth(n: u32) {
    for i in 0..n {
        // print spaces
        for _j in 0..i {
            print!(" ");
        }
        // print stars
        for _j in 0..((2 * (n - i)) - 1) {
            print!("*");
        }
        print!("\n");
    }
}

fn pattern_ninth(n: u32) {
    pattern_seventh(n);
    pattern_eighth(n);
}

fn pattern_tenth(n: u32) {
    for i in 0..((2 * n) - 1) {
        if i < n {
            for _j in 0..(i + 1) {
                print!("*");
            }
        } else {
            for _j in 0..(2 * n - i - 1) {
                print!("*");
            }
        }
        print!("\n");
    }
}

fn main() {
    // println!("Hello, world!");
    // print_square(8);
    // print_right_triangle(5);
    // print_numbered_triangle(5);
    // pattern_fourth(5);
    // pattern_fifth(5);
    // pattern_sixth(5);
    // pattern_seventh(5);
    // pattern_eighth(5);
    // pattern_ninth(5);
    pattern_tenth(5);
}
