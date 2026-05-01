use std::collections::HashMap;

fn main() {
    let s1 = String::from("Hello World");
    let s2 = "Hello World".to_string();
    let s3 = "Hello World";

    println!("{s1:?}\n{s2:?}\n{s3:?}");

    let mut f = "foo".to_string();
    f.push_str("bar");
    let f = f;
    println!("f => {f:?}");

    let mut l = "LO".to_string();
    l.push('L');
    let l = l;
    println!("l => {l:?}");

    let s1 = "Hello".to_string();
    let s2 = "World".to_string();
    let s1 = s1 + " " + &s2;
    println!("s1 s2 => {s1:?}");

    let mut scores: HashMap<String, String> = HashMap::new();
    scores.insert(String::from("Blue"), String::from("12"));

    let test = String::from("0");
    let blue_team_score = scores.get("Blue").unwrap_or(&test);
    println!("{blue_team_score:?}");
}
