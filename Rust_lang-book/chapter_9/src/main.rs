struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    println!("Hello, world!");

    let a: Point<f64> = Point { x: 1.0, y: 2.0 };
}
