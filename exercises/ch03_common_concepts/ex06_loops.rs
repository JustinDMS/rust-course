//! # Labels, break values and bounds
//!
//! Concept: `break` leaves the innermost loop unless it names a label;
//! `break value` makes a `loop` produce a value; array indices are checked.
//!
//! Book: 3.5 "Control Flow"
//!
//! Your job: one function is wrong (its test fails), two do not compile.
//! Fix all three without changing the tests.

/// The indices of the **first** pair (in order of `i`, then `j`) whose
/// values add up to `target`, or `(0, 0)` if there is none.
fn first_pair_summing_to(values: [i32; 6], target: i32) -> (usize, usize) {
    let mut result = (0, 0);
    'outer: for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] + values[j] == target {
                result = (i, j);
                break;
            }
        }
    }
    result
}

/// The last element of a four-element array.
fn last(values: [i32; 4]) -> i32 {
    values[4]
}

/// Doubles `start` until it exceeds `limit`, and returns the number of
/// doublings it took. `doublings_to_exceed(1, 100)` is 7 (128 > 100).
fn doublings_to_exceed(start: u32, limit: u32) -> u32 {
    let mut value = start;
    let mut count = 0;
    let steps = loop {
        if value > limit {
            break;
        }
        value *= 2;
        count += 1;
    };
    steps
}

#[test]
fn finds_the_first_pair() {
    assert_eq!(first_pair_summing_to([1, 2, 3, 4, 5, 6], 7), (0, 5));
    assert_eq!(first_pair_summing_to([5, 5, 5, 5, 5, 5], 10), (0, 1));
    assert_eq!(first_pair_summing_to([1, 2, 3, 4, 5, 6], 100), (0, 0));
}

#[test]
fn last_is_the_fourth_element() {
    assert_eq!(last([1, 2, 3, 4]), 4);
    assert_eq!(last([9, 9, 9, -1]), -1);
}

#[test]
fn counts_doublings() {
    assert_eq!(doublings_to_exceed(1, 100), 7);
    assert_eq!(doublings_to_exceed(50, 100), 2);
    assert_eq!(doublings_to_exceed(200, 100), 0);
}
