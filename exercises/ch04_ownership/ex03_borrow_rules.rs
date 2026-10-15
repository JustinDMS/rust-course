//! # The borrowing rules
//!
//! Concept: at any moment you can have *either* any number of shared
//! references (`&T`) *or* exactly one mutable reference (`&mut T`) to a
//! value, and a reference must never outlive the value it points to.
//!
//! Book: 4.2 "References and Borrowing"
//!
//! Your job: three functions each break one rule. Fix them so the tests
//! compile and pass. Do not change the tests.

/// Appends the double of every existing element.
/// `[1, 2]` becomes `[1, 2, 2, 4]`.
fn push_doubles(values: &mut Vec<i32>) {
    for v in values.iter() {
        values.push(v * 2);
    }
}

/// Returns the longest of the built-in weapon names.
fn longest_weapon_name() -> &String {
    let names = vec![
        String::from("beam"),
        String::from("missile"),
        String::from("bomb"),
    ];
    let mut longest = &names[0];
    for n in &names {
        if n.len() > longest.len() {
            longest = n;
        }
    }
    longest
}

/// Swaps the first and last elements, then pushes the sum of all of them.
fn swap_ends_and_sum(values: &mut Vec<i32>) {
    let first = &mut values[0];
    let last = &mut values[values.len() - 1];
    std::mem::swap(first, last);
    let sum: i32 = values.iter().sum();
    values.push(sum);
}

#[test]
fn doubles_are_appended() {
    let mut v = vec![1, 2, 3];
    push_doubles(&mut v);
    assert_eq!(v, vec![1, 2, 3, 2, 4, 6]);
}

#[test]
fn doubles_of_nothing() {
    let mut v: Vec<i32> = Vec::new();
    push_doubles(&mut v);
    assert!(v.is_empty());
}

#[test]
fn missile_is_longest() {
    assert_eq!(longest_weapon_name(), "missile");
}

#[test]
fn ends_swapped_and_summed() {
    let mut v = vec![1, 5, 9];
    swap_ends_and_sum(&mut v);
    assert_eq!(v, vec![9, 5, 1, 15]);
}
