//! # Modules in separate files
//!
//! Concept: `mod name;` loads `name.rs` (or `name/mod.rs`); a submodule of
//! a file module lives in a directory named after it; a file nobody
//! declares is not compiled at all.
//!
//! Book: 7.5 "Separating Modules into Different Files"
//!
//! This exercise edits the chapter's **library crate**, not this file:
//!
//! - `exercises/ch07_modules/src/lib.rs`
//! - `exercises/ch07_modules/src/inventory.rs`
//! - `exercises/ch07_modules/src/inventory/items.rs`
//!
//! Your job: make the paths used below exist and be public. You will need
//! to declare the `items` submodule in `inventory.rs`, make `inventory`
//! public in `lib.rs`, and expose the `Item` fields the tests read. Do not
//! change this file.

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
