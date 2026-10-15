//! # Labels, break values and bounds (solution)
//!
//! `first_pair_summing_to`: the plain `break` only left the inner loop, so
//! later pairs overwrote the result. `break 'outer` stops both.
//! `last`: indices start at 0, so the fourth element is `values[3]`.
//! `doublings_to_exceed`: a `loop` used as a value must `break` *with* a
//! value.

/// The indices of the first pair whose values add up to `target`.
fn first_pair_summing_to(values: [i32; 6], target: i32) -> (usize, usize) {
    let mut result = (0, 0);
    'outer: for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] + values[j] == target {
                result = (i, j);
                break 'outer;
            }
        }
    }
    result
}

/// The last element of a four-element array.
fn last(values: [i32; 4]) -> i32 {
    values[3]
}

/// Doubles `start` until it exceeds `limit`.
fn doublings_to_exceed(start: u32, limit: u32) -> u32 {
    let mut value = start;
    let mut count = 0;
    let steps = loop {
        if value > limit {
            break count;
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
