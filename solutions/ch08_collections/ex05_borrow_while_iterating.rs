//! # Changing a collection you are looking at (solution)
//!
//! `drop_below`: `retain` is built for exactly this. (An index loop that
//! only advances when nothing was removed also works.)
//! `duplicate_evens`: collect the evens into a second vector, then
//! `extend`. Or loop over `0..values.len()` by index.
//! `apply_bonus`: `iter_mut()` (or `values_mut()`) hands out `&mut` to each
//! value; the insert happens after the loop, when the borrow has ended.

use std::collections::HashMap;

fn drop_below(values: &mut Vec<i32>, min: i32) {
    values.retain(|&v| v >= min);
}

fn duplicate_evens(values: &mut Vec<i32>) {
    let mut evens = Vec::new();
    for &v in values.iter() {
        if v % 2 == 0 {
            evens.push(v);
        }
    }
    values.extend(evens);
}

fn apply_bonus(scores: &mut HashMap<String, i32>, bonus: i32) {
    let mut updated = 0;
    for score in scores.values_mut() {
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
