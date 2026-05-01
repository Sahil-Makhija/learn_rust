// Closures
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.get_most_stocked())
    }

    fn get_most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => blue_count += 1,
                ShirtColor::Red => red_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    fn check_if_available(&self, color: ShirtColor) -> bool {
        self.shirts.contains(&color)
    }
}

fn main() {
    println!("Hello, world!");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue],
    };

    let user_pref1 = None;
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
}
