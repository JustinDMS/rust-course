//! # Coins, Option and if let
//!
//! Concept: patterns bind the data inside a variant; `Option` composes
//! with `match`; `if let` handles one case.
//!
//! Book: 6.2 and 6.3
//!
//! Your job: implement the four functions.

#[derive(Debug, Clone, Copy, PartialEq)]
enum UsState {
    Alabama,
    Alaska,
    // ... the rest are left out
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// 1, 5, 10 or 25.
fn value_in_cents(coin: Coin) -> u8 {
    todo!()
}

/// The state on a quarter, or `None` for any other coin.
fn state_of(coin: Coin) -> Option<UsState> {
    todo!()
}

/// `Some(x + 1)` for `Some(x)`, `None` for `None`.
fn plus_one(x: Option<i32>) -> Option<i32> {
    todo!()
}

/// The total value of the coins, and how many of them were quarters
/// from Alaska. Use `if let` for the second count.
fn tally(coins: &[Coin]) -> (u32, u32) {
    todo!()
}

#[test]
fn values() {
    assert_eq!(value_in_cents(Coin::Penny), 1);
    assert_eq!(value_in_cents(Coin::Nickel), 5);
    assert_eq!(value_in_cents(Coin::Dime), 10);
    assert_eq!(value_in_cents(Coin::Quarter(UsState::Alabama)), 25);
}

#[test]
fn states() {
    assert_eq!(state_of(Coin::Quarter(UsState::Alaska)), Some(UsState::Alaska));
    assert_eq!(state_of(Coin::Dime), None);
}

#[test]
fn plus_one_works() {
    assert_eq!(plus_one(Some(5)), Some(6));
    assert_eq!(plus_one(None), None);
}

#[test]
fn tallies() {
    let coins = [
        Coin::Penny,
        Coin::Quarter(UsState::Alaska),
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ];
    assert_eq!(tally(&coins), (86, 2));
    assert_eq!(tally(&[]), (0, 0));
}
