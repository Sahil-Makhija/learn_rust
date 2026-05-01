#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
}

impl Book {
    fn borrow_book(self) {
        return;
    }

    fn return_book(self) {
        return;
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn add_book(self, book: Book) {
        return;
    }

    fn borrow(self, title: &String) {
        return;
    }

    fn list_available_books(self) {
        return;
    }
}

fn main() {
    println!("Hello, world!");
    let mut library = Library { books: Vec::new() };
}
