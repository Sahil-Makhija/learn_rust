use std::io;
use std::cmp::Ordering;

fn main(){

    println!("Enter a Number!");
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("failed to read line.");

    let count:u32 = match count.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let mut index = 0;
    loop {
        match index.cmp(&count) {
            Ordering::Equal => {
                println!("Loop Completed.");
                break;
            },
            Ordering::Less =>{
                index = index + 1;
                println!("{index}");
            },
            Ordering::Greater => break
        };
    }
    
}