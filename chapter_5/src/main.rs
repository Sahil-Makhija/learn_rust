mod rectangle;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Un-named Structs
struct rgb(i32, i32, i32);
struct origin(i32, i32, i32);

// Unit-like structs
struct AlwaysBlue;

fn return_ball() -> AlwaysBlue {
    let ball = AlwaysBlue;
    ball
}

fn return_color() -> rgb {
    // This functions returns a struct `rgb`.
    rgb(1, 2, 3)

    // If we try to return struct `origin` , the program will error even if the value type is exactly same.
    // return origin(1,2,3)
}

fn main() {
    let user_1 = User {
        active: true,
        username: String::from("Conner"),
        email: String::from("conner@intigriti.me"),
        sign_in_count: 1,
    };

    // To change any value in `user_1` , the whole struct should be mutable
    let mut user_2 = User {
        active: false,
        username: String::from("Sahil"),
        email: String::from("sahilmakhija@intigriti.me"),
        sign_in_count: 1,
    };

    user_2.active = true;

    let user_3 = build_user(String::from("newuser@email.com"), String::from("new_user"));
    println!("User 3 : {}\nemail : {}", user_3.username, user_3.email);

    // Creating Instances from Other Instances with Struct Update Syntax
    let user_4 = User {
        username: String::from("connermck"),
        sign_in_count: 2,
        ..user_1
    };
    // Since we moved string values from user_1 , we no longer can access user_1, as the data has been `moved`.
    println!("Updated username : {}", user_4.username);

    user_2.sign_in_count += 1;
    println!("Signed in successfully.");

    // Destructuring values out of a struct...
    let rgb(r, g, b) = return_color();
    println!("Red : {},\nGreen : {},\nBlue : {}", r, g, b);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
