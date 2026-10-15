//! # Modules in separate files (solution)
//!
//! This file is unchanged; the fixes are in `src/lib.rs` (`pub mod
//! inventory;`), `src/inventory.rs` (`pub mod items;`) and
//! `src/inventory/items.rs` (`pub name`, `pub weight`).

use ch07_modules::inventory::items::Item;
use ch07_modules::inventory::total_weight;

#[test]
fn items_can_be_built_and_read() {
    let item = Item::new("missile", 2);
    assert_eq!(item.name, "missile");
    assert_eq!(item.weight, 2);
}

#[test]
fn weights_add_up() {
    let items = vec![Item::new("missile", 2), Item::new("tank", 10)];
    assert_eq!(total_weight(&items), 12);
    assert_eq!(total_weight(&[]), 0);
}
