//! # Iterator adaptors
//!
//! Concept: `iter()`, then adaptors (`filter`, `map`, `enumerate`, ...),
//! then one consumer (`collect`, `sum`, `max_by_key`, `position`, ...).
//!
//! Book: 13.2 "Processing a Series of Items with Iterators"
//!
//! Your job: implement each function as a single iterator chain. The
//! checker rejects `for` and `while` loops in this file.

/// The squares of the even values, in order: `[1, 2, 3, 4]` → `[4, 16]`.
fn squares_of_evens(values: &[i32]) -> Vec<i32> {
    todo!()
}

/// The total number of characters across all words.
fn total_len(words: &[&str]) -> usize {
    todo!()
}

/// The longest word (the first one on a tie), or `None` for no words.
/// Hint: `split_whitespace()` and `max_by_key`... but note that
/// `max_by_key` returns the *last* maximum on a tie. Use `rev()` first, or
/// compare with `>` in a `fold`.
fn longest_word(text: &str) -> Option<&str> {
    todo!()
}

/// The index of the first negative value, if any.
fn first_negative_index(values: &[i32]) -> Option<usize> {
    todo!()
}

/// Each word upper-cased and numbered from 1, joined with `", "`:
/// `["a", "b"]` → `"1:A, 2:B"`.
fn numbered_upper(words: &[&str]) -> String {
    todo!()
}

#[test]
fn squares() {
    assert_eq!(squares_of_evens(&[1, 2, 3, 4]), vec![4, 16]);
    assert!(squares_of_evens(&[1, 3]).is_empty());
}

#[test]
fn lengths() {
    assert_eq!(total_len(&["ab", "cde", ""]), 5);
    assert_eq!(total_len(&[]), 0);
}

#[test]
fn longest() {
    assert_eq!(longest_word("the quick brown fox"), Some("quick"));
    assert_eq!(longest_word("aa bb cc"), Some("aa"));
    assert_eq!(longest_word("   "), None);
}

#[test]
fn negatives() {
    assert_eq!(first_negative_index(&[3, 0, -2, -5]), Some(2));
    assert_eq!(first_negative_index(&[1, 2]), None);
}

#[test]
fn numbering() {
    assert_eq!(numbered_upper(&["a", "b"]), "1:A, 2:B");
    assert_eq!(numbered_upper(&[]), "");
}
