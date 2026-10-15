//! # Mutability, constants and shadowing
//!
//! Concept: bindings are immutable unless marked `mut`; constants need a
//! type; shadowing (`let x = ...` twice) makes a new binding and may change
//! the type, while assignment (`x = ...`) may not.
//!
//! Book: 3.1 "Variables and Mutability"
//!
//! Your job: three compile errors, one per concept. Fix them without
//! changing the tests.

/// The most health a player can have.
const MAX_HP = 99;

/// 1 + 2 + ... + n
fn sum_to(n: u32) -> u32 {
    let total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

/// Formats a level number as a label: `7` becomes `"Lv7"`.
fn level_label(level: u32) -> String {
    let mut label = level;
    label = format!("Lv{label}");
    label
}

#[test]
fn max_hp_is_99() {
    assert_eq!(MAX_HP, 99);
}

#[test]
fn sum_to_adds_up() {
    assert_eq!(sum_to(0), 0);
    assert_eq!(sum_to(4), 10);
    assert_eq!(sum_to(100), 5050);
}

#[test]
fn level_label_formats() {
    assert_eq!(level_label(7), "Lv7");
    assert_eq!(level_label(42), "Lv42");
}
