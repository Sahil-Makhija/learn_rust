#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn print_rectangle(r: &Rectangle) {
    // To do this, we need to set an outer attribute just before Rectangle definition and add `:?` ahead of the given rectangle.
    // println!("The given rectangle is {r:?}");
    // OR
    dbg!(r);
}

fn calculate_area(r: &Rectangle) -> u32 {
    r.width * r.length
}

fn calculate_perimeter(r: &Rectangle) -> u32 {
    2 * (r.width + r.length)
}

fn main() {
    let r = Rectangle {
        width: 20,
        length: 10,
    };

    println!("Area of rectangle : {}", calculate_area(&r));
    println!("Perimeter of rectangle : {}", calculate_perimeter(&r));

    print_rectangle(&r);
}
