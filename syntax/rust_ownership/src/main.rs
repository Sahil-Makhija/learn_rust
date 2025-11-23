fn main() {
    println!("Hello, world!");

    // let s = "hello"; // Type : String literal ;
    // let m = String::from("world"); // Type : String
    // any_str_func(m);
    // println!("m : {}", m); We cannot use it after sending it to a function

    // let mut s1 = 1;
    // {
    //     let r1 = &mut s1;
    // }

    // let r2 = &mut s1;

    let words = String::from("This is a  sentence.");
    let idx = return_first_word(&words);
    let first_word = &words[0..idx];
    println!("First word is : {first_word}");
}

// fn any_str_func(any_str: String) {
//     println!("any_str : {}", any_str);
// }

fn return_first_word(str: &String) -> usize {
    let bytes = str.as_bytes().iter().enumerate();
    for (i, &byte) in bytes {
        if byte == b' ' {
            return i;
        }
    }
    str.len()
}
