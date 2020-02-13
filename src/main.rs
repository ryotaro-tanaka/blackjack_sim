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
        // one deck game
        let mut _case = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
            ];
        _case.shuffle(&mut rand::thread_rng());
        self.cards = _case;

        // // six deck game
        // let mut _pack = vec![];
        // for _i in 0..6 {
        //     _pack.append(&vec![
        //             1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
        //             1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
        //             1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
        //             1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
        //             ]);
        // }
        // _pack.shuffle(&mut rand::thread_rng());
        // self.cards.append(_pack);
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
            cash: 0,
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
    // init
    let mut deck = Deck::new();
    deck.add();
    let mut _player = Player::new();
    let mut _dealer = Dealer::new();

    // for result window
    let times = 100000;
    let mut win_count = 0;
    let mut lose_count = 0;
    let mut player_bust_count = 0;
    let mut dealer_bust_count = 0;

    // game
    for _i in 0..times {
        //drow 
        _player.hit(deck.drow());
        _player.hit(deck.drow());
        let open_card = deck.drow();
        _dealer.hit(open_card);
        _dealer.hit(deck.drow());

        //think
        while player_thinks(&_player.hand, open_card) {
            _player.hit(deck.drow());
        }
        while dealer_thinks(&_dealer.hand) {
            _dealer.hit(deck.drow());
        }

        //result
        let bet = 10;
        let sum_player = sum(&_player.hand);
        let sum_dealer = sum(&_dealer.hand);

        if sum_player > 21 ||
        (sum_dealer <= 21 && sum_player < sum_dealer) {
            // lose
            _player.pay(-bet);
            lose_count = lose_count + 1;
        } else if sum_dealer > 21 ||
        sum_player > sum_dealer {
            // win
            if check_bj(&_player.hand) {
                _player.pay((bet as f64 * 1.5) as i32); 
            } else {
                _player.pay(bet);
            }
            win_count = win_count + 1;
        } else {
            // no game
            assert!(sum_player <= 21);
            assert!(sum_dealer <= 21);
            assert!(sum_player == sum_dealer);
        }

        // for result window
        if sum_player > 21 {
            player_bust_count = player_bust_count + 1;
        }
        if sum_dealer > 21 {
            dealer_bust_count = dealer_bust_count + 1;
        }

        // next game
        _player.remove_hand();
        _dealer.remove_hand();
    }

    println!(" -------result-------");
    println!("repeated {} times", times);
    println!("rate => win: {}%, lose: {}%", win_count / (times / 100), lose_count / (times / 100));
    println!("cash => {}", _player.cash);
    println!("player bust is {} times", player_bust_count);
    println!("dealer bust is {} times", dealer_bust_count);
}
fn player_thinks(_hand: &Vec<i32>, _open_card: i32) -> bool {
    let sum_val = sum(_hand);
    let mut conv_open_card = _open_card;
    if conv_open_card == 1 { conv_open_card = 11; }

    if sum_val < 12 { return true; }

    // 3: look and think about open card
    // if _open_card < 7 && _open_card != 1 { return false; }
    if conv_open_card < 7 { return false; }

    // 4: look and think about open card and sum value
    if sum_val >= 15 && sum_val < 21 && sum_val < conv_open_card + 10 { return true; }

    // 2: change threshold of hit
    sum_val < 15
}
fn dealer_thinks(_hand: &Vec<i32>) -> bool {
    sum(_hand) < 17
}
fn sum(hand: &Vec<i32>) -> i32 {
    let mut sum = HashMap::new();
    sum.insert("small_ace", 0 as i32);
    sum.insert("big_ace", 0 as i32);

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