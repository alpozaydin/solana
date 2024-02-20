enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn print_publication_details(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => {
                println!("Book: {} \nAuthor: {},\nPage_count {}\n", book.title, book.author, book.page_count);
            },
            Publication::Magazine(magazine) => {
                println!("Magazine: {} \nIssue: {}, \nTopic: {}", magazine.title, magazine.issue, magazine.topic);
            },
        }
    }
}

fn main() {
    let book = Book {
        title: "My Rust Book".to_string(),
        author: "Anonymous".to_string(),
        page_count: 999,
    };

    let magazine = Magazine {
        title: "Rust World".to_string(),
        issue: 10,
        topic: "Computer Design".to_string(),
    };

    let publications = vec![Publication::Book(book), Publication::Magazine(magazine)];

    print_publication_details(publications);
}
