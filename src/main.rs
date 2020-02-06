use rand::seq::SliceRandom;

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
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            ];
        _case.shuffle(&mut rand::thread_rng());
        self.cards = _case;
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.add();

    println!("{}", deck.cards.len());
    println!("{:?}", deck.cards);
    println!("Hello, world!");
}

// == println!("{:?}", _cards);
// fn print_cards(_cards: Vec<i32>) {
//     for card in _cards {
//         print!("{}, ", card);
//     }
//     println!("");
// }
