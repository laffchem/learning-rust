#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author} => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook { title: String::from("An audiobook") };
    let good_movie = Media::Movie { title: String::from("Good Movie"), director: String::from("Good Director") };
    let bad_book = Media::Audiobook { title: String::from("Bad Author") };
    
    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
}
