#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let values = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ]; 

    let mut cards = vec![];
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    let deck = Deck { cards };
    return deck;
    }
}

fn main() {

    let deck = Deck::new();

    println!("Heres your deck: {:#?}", deck);
}
