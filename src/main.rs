struct Deck {
    cards: Vec<i32>,
}
impl Deck {
    fn new () -> Deck {
        Deck {
            cards: vec![],
        }
    }
    fn add(&mut self) {
        let mut _case = vec![
            1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 10, 10, 10
            ];
        self.cards = _case;
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.add();

    print_cards(deck.cards);
    println!("Hello, world!");
}
fn print_cards(_cards: Vec<i32>) {
    for card in _cards {
        print!("{}, ", card);
    }
    println!("");
}
