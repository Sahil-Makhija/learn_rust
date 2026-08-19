use std::{iter, print, println};

fn pattern_01(n: u32) {
    for _i in 0..n {
        for _j in 0..n {
            print!("* ");
        }
        print!("\n");
    }
}

fn pattern_02(n: u32) {
    for i in 0..n {
        for _j in 0..(i + 1) {
            print!("* ");
        }
        print!("\n");
    }
}

fn pattern_03(n: u32) {
    for i in 0..n {
        for j in 1..(i + 2) {
            print!("{}", j);
        }
        print!("\n");
    }
}

fn pattern_04(n: u32) {
    for i in 1..(n + 1) {
        for _j in 0..i {
            print!("{}", i);
        }
        print!("\n");
    }
}

fn pattern_05(n: u32) {
    for i in 0..n {
        for _j in 0..(n - i) {
            print!("*");
        }
        print!("\n");
    }
}

fn pattern_06(n: u32) {
    for i in 0..n {
        for j in 0..(n - i) {
            print!("{}", j + 1)
        }
        print!("\n");
    }
}

fn pattern_07(n: u32) {
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

fn pattern_08(n: u32) {
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

fn pattern_09(n: u32) {
    pattern_07(n);
    pattern_08(n);
}

fn pattern_10(n: u32) {
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
        for _j in 0..(i + 1) {
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
        for j in (n - i)..(n + 1) {
            print!("{}", (b'A' + j - 1) as char);
        }
        println!();
    }
}

fn pattern_19(n: usize) {
    for i in 0..(n) {
        iter::repeat("*")
            .take(n - i)
            .for_each(|star| print!("{star}"));
        iter::repeat(" ")
            .take(i * 2)
            .for_each(|space| print!("{space}"));
        iter::repeat("*")
            .take(n - i)
            .for_each(|star| print!("{star}"));
        println!();
    }
    for i in 0..(n) {
        iter::repeat("*")
            .take(i + 1)
            .for_each(|star| print!("{star}"));
        iter::repeat(" ")
            .take((n - i - 1) * 2)
            .for_each(|space| print!("{space}"));
        iter::repeat("*")
            .take(i + 1)
            .for_each(|star| print!("{star}"));
        println!();
    }
}

fn pattern_20(n: usize) {
    for i in 0..(n) {
        iter::repeat("*")
            .take(i + 1)
            .for_each(|star| print!("{star}"));
        iter::repeat(" ")
            .take((n - i - 1) * 2)
            .for_each(|space| print!("{space}"));
        iter::repeat("*")
            .take(i + 1)
            .for_each(|star| print!("{star}"));
        println!();
    }
    for i in 1..(n) {
        iter::repeat("*")
            .take(n - i)
            .for_each(|star| print!("{star}"));
        iter::repeat(" ")
            .take(i * 2)
            .for_each(|space| print!("{space}"));
        iter::repeat("*")
            .take(n - i)
            .for_each(|star| print!("{star}"));
        println!();
    }
}

fn pattern_21(n: usize) {
    iter::repeat("*")
        .take(n - 1)
        .for_each(|star| print!("{star}"));
    println!();
    for _i in 0..(n - 1 - 2) {
        print!("*");
        iter::repeat(" ")
            .take(n - 1 - 2)
            .for_each(|space| print!("{space}"));
        print!("*");
        println!();
    }
    iter::repeat("*")
        .take(n - 1)
        .for_each(|star| print!("{star}"));
    println!();
}

fn pattern_22(n: usize) {
    let half_count = n - 1;

    // top half
    for i in 0..half_count {
        for j in 0..i {
            print!("{}", n - j);
        }
        iter::repeat(n - i)
            .take((half_count - i) * 2 + 1)
            .for_each(|num| print!("{num}"));

        for j in 0..i {
            print!("{}", n - i + j + 1);
        }
        println!();
    }

    // middle line
    for j in 0..n {
        print!("{}", n - j)
    }
    for j in 1..n {
        print!("{}", j + 1)
    }
    println!();
    // bottom half
    for i in 0..half_count {
        for j in 0..(half_count - i - 1) {
            print!("{}", n - j);
        }
        iter::repeat(i + 2)
            .take(((i + 2) * 2) - 1)
            .for_each(|ch| print!("{ch}"));
        for j in 0..(half_count - i - 1) {
            print!("{}", i + 3 + j);
        }
        println!();
    }
}

fn main() {
    println!("--- Pattern 01 ---");
    pattern_01(8);
    println!("\n--- Pattern 02 ---");
    pattern_02(5);
    println!("\n--- Pattern 03 ---");
    pattern_03(5);
    println!("\n--- Pattern 04 ---");
    pattern_04(5);
    println!("\n--- Pattern 05 ---");
    pattern_05(5);
    println!("\n--- Pattern 06 ---");
    pattern_06(5);
    println!("\n--- Pattern 07 ---");
    pattern_07(5);
    println!("\n--- Pattern 08 ---");
    pattern_08(5);
    println!("\n--- Pattern 09 ---");
    pattern_09(5);
    println!("\n--- Pattern 10 ---");
    pattern_10(5);
    println!("\n--- Pattern 11 ---");
    pattern_11(5);
    println!("\n--- Pattern 12 ---");
    pattern_12(5);
    println!("\n--- Pattern 13 ---");
    pattern_13(5);
    println!("\n--- Pattern 14 ---");
    pattern_14(5);
    println!("\n--- Pattern 15 ---");
    pattern_15(5);
    println!("\n--- Pattern 16 ---");
    pattern_16(5);
    println!("\n--- Pattern 17 ---");
    pattern_17(5);
    println!("\n--- Pattern 18 ---");
    pattern_18(5);
    println!("\n--- Pattern 19 ---");
    pattern_19(5);
    println!("\n--- Pattern 20 ---");
    pattern_20(5);
    println!("\n--- Pattern 21 ---");
    pattern_21(10);
    println!("\n--- Pattern 22 ---");
    pattern_22(9);
}
