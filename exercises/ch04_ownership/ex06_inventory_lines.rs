//! # Parsing an inventory with slices
//!
//! Concept: parsing text by slicing produces `&str`s that borrow from the
//! input, so a whole inventory can be parsed without allocating a single
//! new string.
//!
//! Book: 4.3 "The Slice Type"
//!
//! An inventory file looks like this (one `name:count` entry per line):
//!
//! ```text
//! missile:5
//! energy tank:2
//! power bomb:1
//! ```
//!
//! Your job: implement the three functions. Names may contain spaces and
//! are everything before the first `:`. Counts are decimal integers; you
//! may assume well-formed input (`parse().unwrap()` is fine here; proper
//! error handling arrives in chapter 9). Blank lines are skipped.

/// Splits one `name:count` line into its parts.
/// The returned name borrows from `line`.
fn parse_entry(line: &str) -> (&str, u32) {
    todo!()
}

/// All entries of the inventory text, in order, skipping blank lines.
fn parse_inventory(text: &str) -> Vec<(&str, u32)> {
    todo!()
}

/// The count for `name`, or 0 if the item is not listed.
fn count_of(text: &str, name: &str) -> u32 {
    todo!()
}

const SAMPLE: &str = "missile:5\nenergy tank:2\n\npower bomb:1\n";

#[test]
fn entry_splits_at_first_colon() {
    assert_eq!(parse_entry("missile:5"), ("missile", 5));
    assert_eq!(parse_entry("energy tank:12"), ("energy tank", 12));
    assert_eq!(parse_entry("x:0"), ("x", 0));
}

#[test]
fn inventory_lists_every_entry() {
    assert_eq!(
        parse_inventory(SAMPLE),
        vec![("missile", 5), ("energy tank", 2), ("power bomb", 1)]
    );
    assert!(parse_inventory("").is_empty());
    assert!(parse_inventory("\n\n").is_empty());
}

#[test]
fn counts_are_looked_up_by_name() {
    assert_eq!(count_of(SAMPLE, "energy tank"), 2);
    assert_eq!(count_of(SAMPLE, "missile"), 5);
    assert_eq!(count_of(SAMPLE, "grapple beam"), 0);
}

#[test]
fn names_borrow_from_the_text() {
    let text = String::from("beam:1");
    let entries = parse_inventory(&text);
    assert_eq!(entries[0].0.as_ptr(), text.as_ptr());
}
