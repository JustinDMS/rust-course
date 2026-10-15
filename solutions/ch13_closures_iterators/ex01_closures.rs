//! # Capturing correctly (solution)
//!
//! `adder`: the closure outlives `n`'s stack slot, so it must `move`.
//! `for_each_value`: the test's closure mutates `total`, so the bound must
//! allow mutation (`FnMut`) and the parameter must be `mut f`.
//! `snapshot_then_extend`: the closure held a shared borrow of `list`
//! across the `push`. Take the snapshot first, then extend.

fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn for_each_value<F: FnMut(i32)>(values: &[i32], mut f: F) {
    for &v in values {
        f(v);
    }
}

fn snapshot_then_extend(list: &mut Vec<i32>) -> String {
    let render = || format!("{list:?}");
    let before = render();
    list.push(99);
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
