//! Chapter 7 exercises live in the `exNN_*.rs` files next to `src/`.
//! Each is its own test target: `cargo test -p ch07_modules --test ex01_privacy`.
//!
//! Exercise 4 is different: it uses this library crate. Its tests in
//! `ex04_files.rs` expect to reach `ch07_modules::inventory::items::Item`
//! and `ch07_modules::inventory::total_weight`. Make that path exist by
//! declaring the modules (`src/inventory.rs` and `src/inventory/items.rs`
//! are already written) and making the right things `pub`.

mod inventory;
