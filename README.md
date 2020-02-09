# blackjack_sim
[![license](https://img.shields.io/github/license/ryotaro-tanaka/blackjack_sim "license")](https://github.com/ryotaro-tanaka/blackjack_sim/blob/master/LICENSE "MIT")  
I want to win at BlackJack!  

# Project setup
```
$ cargo run
```

# Rules of BlackJack
[rules](https://bicyclecards.com/how-to-play/blackjack/)  
* The one-deck game.(52 cards)

# Rules of Player
* Have 1000(USD?).
* Bet is 10(USD?).

# Best practice of player action
1. Hit if less than to 12.
1. Stand if the dealer has greater than equal to 7.(including A)
1. Stand if greater than or equal to 15.
1. Hit if less than to 15.

# Result (process up to best practice)
## 1: same as dealer
1. Hit if less than 17.

**code**
```rust
fn player_thinks(_hand: &Vec<i32>, 
_open_card: i32) -> bool {
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
1. Hit if less than ?.
**code**
ditto
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

# 3: look at open card and think
1. Hit if less than to 12.
1. Stand if the dealer has greater than equal to 7.(including A)
1. Hit if less than to 15.