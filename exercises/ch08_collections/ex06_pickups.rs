//! # A pickup log
//!
//! Concept: counting with `HashMap` and the entry API, turning a map into a
//! sorted `Vec`, and choosing a winner with ties broken deterministically.
//!
//! Book: 8.3 "Storing Keys with Associated Values in Hash Maps"
//!
//! The game logs every item the player picks up, one name per line. Blank
//! lines and surrounding whitespace are ignored.
//!
//! Your job: implement the three functions.

use std::collections::HashMap;

/// How many times each item was picked up.
fn count_pickups(log: &str) -> HashMap<String, u32> {
    todo!()
}

/// Items with their counts, most frequent first; ties broken by name
/// (alphabetical).
fn ranked(counts: &HashMap<String, u32>) -> Vec<(String, u32)> {
    todo!()
}

/// The most picked-up item and its count, or `None` for an empty log.
/// Ties go to the alphabetically first name.
fn favourite(log: &str) -> Option<(String, u32)> {
    todo!()
}

const LOG: &str = "missile\nenergy\n\nmissile\n  power bomb \nenergy\nmissile\n";

#[test]
fn counting() {
    let counts = count_pickups(LOG);
    assert_eq!(counts.len(), 3);
    assert_eq!(counts["missile"], 3);
    assert_eq!(counts["energy"], 2);
    assert_eq!(counts["power bomb"], 1);
    assert!(count_pickups("\n\n").is_empty());
}

#[test]
fn ranking() {
    let counts = count_pickups(LOG);
    assert_eq!(
        ranked(&counts),
        vec![
            (String::from("missile"), 3),
            (String::from("energy"), 2),
            (String::from("power bomb"), 1),
        ]
    );
    let tie = count_pickups("b\na\nb\na");
    assert_eq!(ranked(&tie), vec![(String::from("a"), 2), (String::from("b"), 2)]);
}

#[test]
fn favourites() {
    assert_eq!(favourite(LOG), Some((String::from("missile"), 3)));
    assert_eq!(favourite("b\na"), Some((String::from("a"), 1)));
    assert_eq!(favourite(""), None);
}
