//! # Loot pipelines (solution)

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

fn rare_names(items: &[Item]) -> Vec<&str> {
    items
        .iter()
        .filter(|i| i.rarity >= Rarity::Rare)
        .map(|i| i.name.as_str())
        .collect()
}

fn total_weight(items: &[Item]) -> u32 {
    items.iter().map(|i| i.weight).sum()
}

fn most_valuable(items: &[Item]) -> Option<&Item> {
    // max_by_key keeps the last maximum; reverse so the first wins ties.
    items.iter().rev().max_by_key(|i| i.value)
}

fn with_prices<'a>(items: &'a [Item], prices: &[u32]) -> Vec<(&'a str, u32)> {
    items
        .iter()
        .zip(prices.iter())
        .map(|(i, &p)| (i.name.as_str(), p))
        .collect()
}

fn pack(items: &[Item], capacity: u32) -> Vec<&Item> {
    items
        .iter()
        .scan(0, |carried, i| {
            *carried += i.weight;
            Some((*carried, i))
        })
        .take_while(|(carried, _)| *carried <= capacity)
        .map(|(_, i)| i)
        .collect()
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
