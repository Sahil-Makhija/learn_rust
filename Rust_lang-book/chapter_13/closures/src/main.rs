use std::{thread, time::Duration};

// impl<T> Optio<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(val) => val,
//             None => f(),
//         }
//     }
// }
// fn main() {
//     println!("Hello, world!");
//     // generate_workout(22, 7);

//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let mut borrows_mutably = || list.push(7);

//     borrows_mutably();
//     println!("After calling closure: {list:?}");

//     let arr = vec![1, 2, 3];
//     // Moved ownership to closure
//     thread::spawn(move || println!("From thread: {arr:?}"));
//     // Cannot access anymore as no longer the owner.
//     // println!("{arr:?}");
// }

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

// fn generate_workout(intensity: u32, random_num: u32) {
//     let exp_closure = |num: u32| {
//         println!("(Expensive calculation)");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if intensity < 25 {
//         println!("Today, do {} pushups.", exp_closure(intensity));
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}
