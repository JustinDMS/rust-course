//! # The borrowing rules (solution)
//!
//! `push_doubles`: iterating borrows the vector immutably while `push` needs
//! it mutably. Loop over indices instead (each `values[i]` read ends before
//! the push), or collect the doubles first and `extend`.
//!
//! `longest_weapon_name`: `names` dies at the end of the function, so no
//! reference into it can be returned. Return an owned `String`.
//!
//! `swap_ends_and_sum`: two `&mut` into the same vector at once. `swap` on
//! the slice does the job with one borrow, and the sum is computed after the
//! mutable borrows have ended.

/// Appends the double of every existing element.
/// `[1, 2]` becomes `[1, 2, 2, 4]`.
fn push_doubles(values: &mut Vec<i32>) {
    let n = values.len();
    for i in 0..n {
        let doubled = values[i] * 2;
        values.push(doubled);
    }
}

/// Returns the longest of the built-in weapon names.
fn longest_weapon_name() -> String {
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
    longest.clone()
}

/// Swaps the first and last elements, then pushes the sum of all of them.
fn swap_ends_and_sum(values: &mut Vec<i32>) {
    let last = values.len() - 1;
    values.swap(0, last);
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
