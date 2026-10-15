//! # Parsing an inventory with slices (solution)
//!
//! `parse_entry` slices around the first colon; both halves borrow from the
//! line. `parse_inventory` builds on it, and `count_of` builds on that.

/// Splits one `name:count` line into its parts.
fn parse_entry(line: &str) -> (&str, u32) {
    let colon = line.find(':').expect("entry has a colon");
    let name = &line[..colon];
    let count = line[colon + 1..].trim().parse().expect("count is a number");
    (name, count)
}

/// All entries of the inventory text, in order, skipping blank lines.
fn parse_inventory(text: &str) -> Vec<(&str, u32)> {
    let mut entries = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        entries.push(parse_entry(line));
    }
    entries
}

/// The count for `name`, or 0 if the item is not listed.
fn count_of(text: &str, name: &str) -> u32 {
    for (item, count) in parse_inventory(text) {
        if item == name {
            return count;
        }
    }
    0
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
