enum Publication {
    Book(Book),
    Magazine(Magazine)
}

struct Book {
     title: String,
     author: String,
     page_count: u32
 }

 struct  Magazine {
     title: String,
     issue: u16,
     topic: String
 }

 fn publish(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => 
                println!("Book: {} - written by {} - {} pages", book.title, book.author, book.page_count),
            Publication::Magazine(magazine) =>  
                println!("Magazine: {}, Issue #{}: {}", magazine.title, magazine.issue, magazine.topic),
        }
    };
 }

fn main() {
    let book = Book {
        title: "Futuraba".to_string(),
        author: "John Doe".to_string(),
        page_count: 246,
    };

    let magazine  = Magazine{
        title:"Astonishing Scents".to_string(),
        issue:5,
        topic: "Best 100 Scents".to_string()
    };

    let publication = vec![Publication::Book(book), Publication::Magazine((magazine))];
    publish(publication);
}
