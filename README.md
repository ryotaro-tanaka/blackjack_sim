# blackjack_sim
[![license](https://img.shields.io/github/license/ryotaro-tanaka/blackjack_sim "license")](https://github.com/ryotaro-tanaka/blackjack_sim/blob/master/LICENSE "MIT")  
I want to win at BlackJack!  

# Project setup
```
$ cargo run
```

# Rules of BlackJack
> [rules](https://bicyclecards.com/how-to-play/blackjack/)  
* The one-deck game.(52 cards)

# Rules of Player
* Have 1000(USD?).
* Bet is 10(USD?).

# Best practice of player action
1. Hit if less than to 12.
1. Stand if the dealer has greater than equal to 7.(including A)
1. Stand if greater than or equal to 15.
1. Hit if less than to 15.

# feature
* add rule.
* bet logic.
* machine Learning.(please, someone teach me...)

# Result (process up to best practice)
## 1: same as dealer
**think**
1. Hit if less than 17.

**code**  
```rust
fn player_thinks(_hand: &Vec<i32>, _open_card: i32) -> bool {
    let sum_val = sum(_hand);
    sum_val < 17
}
```
**result**
```
$ cargo run
 -------result-------
repeated 100000 times
rate => win: 40%, lose: 48%
cash => -58165
player bust is 28364 times
dealer bust is 28330 times
```

## 2: change threshold of hit
**think**
1. Hit if less than ?.

**code**  
```
ditto  
```

**result**  
15 is best?
* 21
```
rate => win: 15%, lose: 82%
cash => -641495
player bust is 82026 times
```
* 20
```
rate => win: 29%, lose: 65%
cash => -336730
player bust is 62482 times
```
* 19
```
rate => win: 36%, lose: 56%
cash => -182150
player bust is 49272 times
```
* 18
```
rate => win: 39%, lose: 51%
cash => -95765
player bust is 38163 times
```
* 17
```
rate => win: 40%, lose: 48%
cash => -58800
player bust is 28233 times
```
* 16
```
rate => win: 41%, lose: 49%
cash => -56385
player bust is 20338 times
```
* 15 (best?)
```
rate => win: 41%, lose: 49%
cash => -53220
player bust is 13282 times
```
* 14
```
rate => win: 42%, lose: 50%
cash => -59400
player bust is 7902 times
dealer bust is 28143 times
```
* 13
```
rate => win: 41%, lose: 50%
cash => -71445
player bust is 3333 times
```
* 12
```
rate => win: 41%, lose: 51%
cash => -81845
player bust is 0 times
```

## 3: look and think about open card
**think**
1. Hit if less than to 12.(no bust)
1. Stand if open card is greater than equal to ?.(Ace = 11)
1. Hit if less than to 15.

**code**
```rust
fn player_thinks(_hand: &Vec<i32>, _open_card: i32) -> bool {
    let sum_val = sum(_hand);

    if sum_val < 12 { return true; }

    // look and think about open card
    if _open_card < ? && _open_card != 1 { return false; }

    sum_val < 15
}
```

**result**
7 is best?
* 10
```
rate => win: 42%, lose: 50%
cash => -62455
player bust is 5030 times
```
* 9
```
rate => win: 42%, lose: 50%
cash => -60230
player bust is 6092 times
```
* 8
```
rate => win: 42%, lose: 49%
cash => -53465
player bust is 7154 times
```
* 7
```
rate => win: 42%, lose: 49%
cash => -47935
player bust is 8230 times
```
* 6
```
rate => win: 42%, lose: 49%
cash => -53165
player bust is 9297 times
```
* 5
```
rate => win: 42%, lose: 49%
cash => -52025
player bust is 10242 times
```
* 4
```
rate => win: 41%, lose: 49%
cash => -56800
player bust is 11244 times
```
* 3
```
rate => win: 42%, lose: 49%
cash => -53845
player bust is 12191 times
```
* 2
```
rate => win: 41%, lose: 49%
cash => -54305
player bust is 13253 times
```

## 4: look and think about open card and sum value
**zentei**  
1. Ace == 11.
2. next card is 10

**think**  
1. Hit if less than to 12.
1. Stand if open card is greater than equal to 7.
1. if sum_val >= 15 && sum_val < 18 && sum_val < conv_open_card + 10 { return true; }
1. Hit if less than to 15.

**code**
```rust
fn player_thinks(_hand: &Vec<i32>, _open_card: i32) -> bool {
    let sum_val = sum(_hand);
    let mut conv_open_card = _open_card;
    if conv_open_card == 1 { conv_open_card = 11; }

    if sum_val < 12 { return true; }

    // 3: look and think about open card
    if conv_open_card < 7 { return false; }

    // 4: look and think about open card and sum value
    if sum_val >= 15 && sum_val < ? && sum_val < conv_open_card + 10 { return true; }

    // 2: change threshold of hit
    sum_val < 15
}
```

**result**
* 21
```
rate => win: 38%, lose: 53%
cash => -124660
player bust is 34607 times
```
* 20
```
rate => win: 39%, lose: 52%
cash => -105965
player bust is 32927 times
```
* 19
```
rate => win: 41%, lose: 50%
cash => -64655
player bust is 27730 times
```
* 18
```
rate => win: 42%, lose: 48%
cash => -42625
player bust is 22896 times
```
* 17
```
rate => win: 42%, lose: 48%
cash => -30915
player bust is 17569 times
```
* 16
```
rate => win: 42%, lose: 48%
cash => -35390
player bust is 12472 times
```
* 15
```
rate => win: 42%, lose: 49%
cash => -50435
player bust is 8159 times
```

## 5: split?
