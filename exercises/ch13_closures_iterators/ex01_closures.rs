//! # Capturing correctly
//!
//! Concept: closures borrow, mutably borrow, or move what they use; `move`
//! forces ownership; `Fn` / `FnMut` / `FnOnce` describe what a caller may
//! do with a closure.
//!
//! Book: 13.1 "Closures"
//!
//! Your job: three compile errors. Fix them without changing the tests.

/// A closure that adds `n` to its argument. Returned to the caller, so it
/// must own `n`.
fn adder(n: i32) -> impl Fn(i32) -> i32 {
    |x| x + n
}

/// Calls `f` once for each value, in order.
fn for_each_value<F: Fn(i32)>(values: &[i32], f: F) {
    for &v in values {
        f(v);
    }
}

/// The values of `list`, formatted, while `list` is also extended.
fn snapshot_then_extend(list: &mut Vec<i32>) -> String {
    let render = || format!("{list:?}");
    list.push(99);
    let before = render();
    before
}

#[test]
fn adders_remember_their_n() {
    let add5 = adder(5);
    let add10 = adder(10);
    assert_eq!(add5(1), 6);
    assert_eq!(add10(1), 11);
    assert_eq!(add5(2), 7);
}

#[test]
fn for_each_value_can_accumulate() {
    let mut total = 0;
    for_each_value(&[1, 2, 3], |v| total += v);
    assert_eq!(total, 6);
}

#[test]
fn snapshot_is_taken_before_extending() {
    let mut list = vec![1, 2];
    assert_eq!(snapshot_then_extend(&mut list), "[1, 2]");
    assert_eq!(list, vec![1, 2, 99]);
}
