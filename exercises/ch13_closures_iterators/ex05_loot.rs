//! # Loot pipelines
//!
//! Concept: iterator chains over a slice of structs: filtering by an
//! enum, summing a field, picking a maximum, pairing with another slice,
//! and a running total with `scan` + `take_while`.
//!
//! Book: 13.2 and 13.3
//!
//! Your job: implement the five functions as iterator chains (no `for` or
//! `while`).

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Rarity {
    Common,
    Rare,
    Legendary,
}

#[derive(Debug, PartialEq)]
struct Item {
    name: String,
    rarity: Rarity,
    weight: u32,
    value: u32,
}

fn item(name: &str, rarity: Rarity, weight: u32, value: u32) -> Item {
    Item { name: name.to_string(), rarity, weight, value }
}

/// Names of every item that is at least `Rare`, in order.
fn rare_names(items: &[Item]) -> Vec<&str> {
    todo!()
}

/// The sum of all weights.
fn total_weight(items: &[Item]) -> u32 {
    todo!()
}

/// The most valuable item (the first one on a tie), or `None`.
fn most_valuable(items: &[Item]) -> Option<&Item> {
    todo!()
}

/// Each item's name paired with the price from `prices` at the same
/// index. Extra items or prices are dropped.
fn with_prices<'a>(items: &'a [Item], prices: &[u32]) -> Vec<(&'a str, u32)> {
    todo!()
}

/// Items taken in order while the running total weight stays within
/// `capacity`. Stops at the first item that would not fit.
/// Hint: `scan` to carry the running weight, `take_while`, then `map`.
fn pack(items: &[Item], capacity: u32) -> Vec<&Item> {
    todo!()
}

fn sample() -> Vec<Item> {
    vec![
        item("rock", Rarity::Common, 5, 1),
        item("varia suit", Rarity::Legendary, 20, 500),
        item("missile", Rarity::Common, 1, 10),
        item("ice beam", Rarity::Rare, 8, 300),
        item("energy tank", Rarity::Rare, 12, 300),
    ]
}

#[test]
fn rare_and_above() {
    assert_eq!(rare_names(&sample()), vec!["varia suit", "ice beam", "energy tank"]);
    assert!(rare_names(&[]).is_empty());
}

#[test]
fn weights() {
    assert_eq!(total_weight(&sample()), 46);
    assert_eq!(total_weight(&[]), 0);
}

#[test]
fn valuables() {
    let items = sample();
    assert_eq!(most_valuable(&items).map(|i| i.name.as_str()), Some("varia suit"));
    let tie = vec![item("a", Rarity::Common, 1, 7), item("b", Rarity::Common, 1, 7)];
    assert_eq!(most_valuable(&tie).map(|i| i.name.as_str()), Some("a"));
    assert_eq!(most_valuable(&[]), None);
}

#[test]
fn pricing() {
    let items = sample();
    assert_eq!(with_prices(&items, &[1, 2, 3]), vec![("rock", 1), ("varia suit", 2), ("missile", 3)]);
    assert!(with_prices(&items, &[]).is_empty());
}

#[test]
fn packing() {
    let items = sample();
    let names: Vec<&str> = pack(&items, 26).iter().map(|i| i.name.as_str()).collect();
    assert_eq!(names, vec!["rock", "varia suit", "missile"]);
    assert!(pack(&items, 4).is_empty());
    assert_eq!(pack(&items, 1000).len(), 5);
}
