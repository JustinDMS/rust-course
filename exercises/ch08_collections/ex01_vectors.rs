//! # Median and mode
//!
//! Concept: `Vec` basics: building, sorting, indexing, iterating; plus a
//! first taste of `HashMap` for counting.
//!
//! Book: 8.1 "Storing Lists of Values with Vectors" (and its summary
//! exercise)
//!
//! Your job: implement the three functions.

use std::collections::HashMap;

/// The median of the values: after sorting, the middle one, or the mean
/// of the two middle ones for an even count. `None` for an empty slice.
/// Hint: `let mut sorted = values.to_vec(); sorted.sort();`
fn median(values: &[i32]) -> Option<f64> {
    todo!()
}

/// The value that occurs most often. On a tie, the smallest such value.
/// `None` for an empty slice.
fn mode(values: &[i32]) -> Option<i32> {
    todo!()
}

/// Only the even values, in their original order.
fn evens(values: &[i32]) -> Vec<i32> {
    todo!()
}

#[test]
fn medians() {
    assert_eq!(median(&[3, 1, 2]), Some(2.0));
    assert_eq!(median(&[4, 1, 3, 2]), Some(2.5));
    assert_eq!(median(&[7]), Some(7.0));
    assert_eq!(median(&[]), None);
}

#[test]
fn modes() {
    assert_eq!(mode(&[1, 2, 2, 3]), Some(2));
    assert_eq!(mode(&[5, 5, 1, 1, 3]), Some(1));
    assert_eq!(mode(&[9]), Some(9));
    assert_eq!(mode(&[]), None);
}

#[test]
fn even_values() {
    assert_eq!(evens(&[1, 2, 3, 4, 6, 7]), vec![2, 4, 6]);
    assert!(evens(&[1, 3, 5]).is_empty());
}
