//! # Coins, Option and if let (solution)

#[derive(Debug, Clone, Copy, PartialEq)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn state_of(coin: Coin) -> Option<UsState> {
    match coin {
        Coin::Quarter(state) => Some(state),
        _ => None,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn tally(coins: &[Coin]) -> (u32, u32) {
    let mut total = 0;
    let mut alaska = 0;
    for &coin in coins {
        total += value_in_cents(coin) as u32;
        if let Coin::Quarter(UsState::Alaska) = coin {
            alaska += 1;
        }
    }
    (total, alaska)
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
