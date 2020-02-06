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
    fn drow(&mut self) -> i32 {
        let card = self.cards[0];
        self.cards.remove(0);

        if self.cards.len() == 0 {
            self.add();
        }

        return card;
    }
}

struct Player {
    hand: Vec<i32>,
    money: i32,
}
impl Player {
    fn new () -> Player {
        Player {
            hand: vec![],
            money: 1000,
        }
    }
    // not considering Ace
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for card in &self.hand {
            sum = sum + card;
        }
        return sum;
    }
    fn hit(&mut self, card: i32) {
        self.hand.push(card);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.add();

    println!("{:?}", deck.cards);
    assert!(deck.cards.len() == 52);

    let mut _player = Player::new();
    _player.hit(deck.drow());

    println!("{}", _player.sum());
    assert!(deck.cards.len() == 51);
}
