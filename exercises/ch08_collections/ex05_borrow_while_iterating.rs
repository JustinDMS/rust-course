//! # Changing a collection you are looking at
//!
//! Concept: iterating borrows the collection, so you cannot push, remove
//! or insert during the loop. Collect first, then change; or use indices;
//! or use an API designed for it (`retain`, `entry`).
//!
//! Book: 8.1 and 8.3
//!
//! Your job: three functions do not compile. Fix them without changing
//! the tests.

use std::collections::HashMap;

/// Removes every value below `min`, in place.
fn drop_below(values: &mut Vec<i32>, min: i32) {
    for (i, v) in values.iter().enumerate() {
        if *v < min {
            values.remove(i);
        }
    }
}

/// Appends a copy of every value that is even, so `[1, 2, 3, 4]` becomes
/// `[1, 2, 3, 4, 2, 4]`.
fn duplicate_evens(values: &mut Vec<i32>) {
    for v in values.iter() {
        if v % 2 == 0 {
            values.push(*v);
        }
    }
}

/// Adds `bonus` to every score, and inserts a `"bonus"` entry recording
/// how many scores were updated.
fn apply_bonus(scores: &mut HashMap<String, i32>, bonus: i32) {
    let mut updated = 0;
    for (_, score) in scores.iter() {
        *score += bonus;
        updated += 1;
    }
    scores.insert(String::from("bonus"), updated);
}

#[test]
fn dropping_small_values() {
    let mut v = vec![5, 1, 7, 2, 9];
    drop_below(&mut v, 5);
    assert_eq!(v, vec![5, 7, 9]);
    let mut all_small = vec![1, 1, 1];
    drop_below(&mut all_small, 2);
    assert!(all_small.is_empty());
}

#[test]
fn duplicating_evens() {
    let mut v = vec![1, 2, 3, 4];
    duplicate_evens(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 2, 4]);
}

#[test]
fn bonuses() {
    let mut scores = HashMap::new();
    scores.insert(String::from("a"), 10);
    scores.insert(String::from("b"), 20);
    apply_bonus(&mut scores, 5);
    assert_eq!(scores["a"], 15);
    assert_eq!(scores["b"], 25);
    assert_eq!(scores["bonus"], 2);
}
