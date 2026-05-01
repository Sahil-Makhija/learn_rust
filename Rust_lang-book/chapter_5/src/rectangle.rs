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

// Implementing methods in struct `Rectangle`
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.length)
    }
}

fn main() {
    let r = Rectangle {
        width: 20,
        length: 10,
    };

    print_rectangle(&r);

    // println!("Area of rectangle : {}", calculate_area(&r));
    // println!("Perimeter of rectangle : {}", calculate_perimeter(&r));

    println!("Area of rectangle : {}", r.area());
    println!("Perimeter of rectangle : {}", r.perimeter());
}
