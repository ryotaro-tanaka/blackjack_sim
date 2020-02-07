use rand::seq::SliceRandom;
use std::collections::HashMap;

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

        card
    }
}

struct Player {
    hand: Vec<i32>,
    cash: i32,
}
impl Player {
    fn new () -> Player {
        Player {
            hand: vec![],
            cash: 1000,
        }
    }
    fn hit(&mut self, card: i32) {
        self.hand.push(card);
    }
    fn remove_hand(&mut self) {
        self.hand = vec![];
    }
    fn pay(&mut self, money: i32) {
        self.cash = self.cash + money;
    }
}

struct Dealer {
    hand: Vec<i32>,
}
impl Dealer {
    fn new() -> Dealer {
        Dealer {
            hand: vec![],
        }
    }
    fn hit(&mut self, card: i32) {
        self.hand.push(card);
    }
    fn remove_hand(&mut self) {
        self.hand = vec![];
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.add();

    let mut _player = Player::new();
    let mut _dealer = Dealer::new();

    println!("{:?}", deck.cards);
    // assert!(deck.cards.len() == 52);

    // game
    for _i in 0..1 {
        //drow 
        _player.hit(deck.drow());
        _player.hit(deck.drow());
        let open_card = deck.drow();
        _dealer.hit(open_card);
        _dealer.hit(deck.drow());

        //think
        while player_thinks() {
            _player.hit(deck.drow());
        }
        while dealer_thinks() {
            _dealer.hit(deck.drow());
        }

        //result
        let bet = 10;
        let sum_player = sum(&_player.hand);
        let sum_dealer = sum(&_dealer.hand);

        if sum_player > 21 || sum_player < sum_dealer {
            _player.pay(-bet);
        } else if sum_player > sum_dealer {
            if check_bj(&_player.hand) {
                _player.pay((bet as f64 * 1.5) as i32);
            } else {
                _player.pay(bet);
            }
        }

        println!("cash => {}", _player.cash);
        println!("player => {} {:?}", sum_player, _player.hand);
        println!("dealer => {} {:?}", sum_dealer, _dealer.hand);

        // next game
        _player.remove_hand();
        _dealer.remove_hand();
    }
}
fn player_thinks() -> bool {
    //
    false
}
fn dealer_thinks() -> bool {
    //
    false
}
// considering Ace
fn sum(hand: &Vec<i32>) -> i32 {
    let mut sum = HashMap::new();
    sum.insert("small_ace", 0 as i32);
    sum.insert("big_ace", 0 as i32);
    // assert!(sum.contains_key("small_ace"));

    let mut is_used_ace = false;
    for card in hand {
        let mut card_val = card;
        sum.insert("small_ace", sum.get("small_ace").unwrap() + card_val);
        
        if card == &1 && !is_used_ace {
            card_val = &11;
            is_used_ace = true;
        }
        sum.insert("big_ace", sum.get("big_ace").unwrap() + card_val);
    }

    let mut result_val = *sum.get("big_ace").unwrap();
    if result_val > 21 {
        result_val = *sum.get("small_ace").unwrap();
    }
    result_val
}
fn check_bj(hand: &Vec<i32>) -> bool {
    if hand.len() != 2 { return false; }
    if sum(hand) != 21 { return false; }

    true
}