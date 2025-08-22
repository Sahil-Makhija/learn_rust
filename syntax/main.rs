
fn main(){
    let mut var = 1;
    println!("Variable value : {var}");
    println!("Changing value to another type...");
    var = "Its string now."; //does not work, because type var is number
    println!("Variable value : {var}");
}