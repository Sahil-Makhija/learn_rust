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

fn pattern_11(n: u32) {
    for i in 0..n {
        for j in 0..(i + 1) {
            print!("{} ", ((i + j + 1) % 2));
        }
        print!("\n");
    }
}

fn pattern_12(n: u32) {
    for i in 0..(n - 1) {
        for j in 1..n {
            if j <= (i + 1) {
                print!("{}", j);
            } else {
                print!(" ");
            }
        }
        for j in 1..n {
            if (n - j) > (i + 1) {
                print!(" ");
            } else {
                print!("{}", n - j);
            }
        }
        print!("\n");
    }
}

fn pattern_13(n: u32) {
    // 4 --> 10
    // 3 --> 6
    // 2 --> 3
    // 1 --> 1
    // 0 --> 0
    let mut cf = 0;
    for i in 0..n {
        cf += i;
        for j in 0..(i + 1) {
            print!("{}", cf + j + 1);
        }
        print!("\n");
    }
}

fn pattern_14(n: u8) {
    // print!("{}", (b'A' + 4) as char);
    for i in 0..n {
        for j in 0..(i + 1) {
            print!("{}", (b'A' + j) as char);
        }
        print!("\n");
    }
}

fn pattern_15(n: u8) {
    // print!("{}", (b'A' + 4) as char);
    for i in 0..n {
        for j in 0..(n - i) {
            print!("{}", (b'A' + j) as char);
        }
        print!("\n");
    }
}

fn pattern_16(n: u8) {
    // print!("{}", (b'A' + 4) as char);
    for i in 0..n {
        for j in 0..(i + 1) {
            print!("{}", (b'A' + i) as char);
        }
        print!("\n");
    }
}

fn pattern_17(n: u8) {
    for i in 0..n {
        for _j in 0..(n - 1 - i) {
            print!(" ");
        }
        for j in 0..(i + 1) {
            print!("{}", (b'A' + j) as char);
        }
        for j in 0..i {
            print!("{}", (b'A' + i - j - 1) as char);
        }
        print!("\n");
    }
}

fn pattern_18(n: u8) {
    for i in 0..n {
        for j in 0..(i + 1) {
            print!("{}", (b'A' - 1 + n - i + j) as char);
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
    // pattern_tenth(5);
    // pattern_11(5);
    // pattern_12(5);
    // pattern_13(5);
    // pattern_14(5);
    // pattern_15(5);
    // pattern_16(5);
    // pattern_17(5);
    pattern_18(5);
}
