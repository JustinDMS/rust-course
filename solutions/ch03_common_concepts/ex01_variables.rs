//! # Mutability, constants and shadowing (solution)
//!
//! `const` needs its type; `total` is changed so it must be `mut`; and
//! `label` changes *type* (u32 to String), which assignment cannot do but a
//! fresh `let` (shadowing) can.

/// The most health a player can have.
const MAX_HP: u32 = 99;

/// 1 + 2 + ... + n
fn sum_to(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

/// Formats a level number as a label: `7` becomes `"Lv7"`.
fn level_label(level: u32) -> String {
    let label = level;
    let label = format!("Lv{label}");
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
