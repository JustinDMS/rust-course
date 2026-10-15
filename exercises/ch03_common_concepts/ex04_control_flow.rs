//! # Loops and conditions
//!
//! Concept: `loop` with `break value`, `while` with a condition, `for` over
//! a range, and `if` as an expression.
//!
//! Book: 3.5 "Control Flow"
//!
//! Your job: implement the four functions. Each suggests a loop; use
//! whichever reads best to you.

/// The n-th Fibonacci number: fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2).
/// A `for` loop over `0..n` that keeps the last two values is enough.
fn fib(n: u32) -> u64 {
    todo!()
}

/// Number of steps the Collatz sequence takes to reach 1 from `n`:
/// if `n` is even, `n / 2`; otherwise `3 * n + 1`. `collatz_steps(1)` is 0,
/// `collatz_steps(6)` is 8 (6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1).
/// A `while n != 1` loop fits.
fn collatz_steps(mut n: u64) -> u32 {
    todo!()
}

/// The sum of all odd numbers strictly below `limit`: `sum_odds_below(10)`
/// is 1 + 3 + 5 + 7 + 9 = 25.
fn sum_odds_below(limit: u32) -> u32 {
    todo!()
}

/// A countdown string: `countdown(3)` is `"3, 2, 1, liftoff!"`, and
/// `countdown(0)` is `"liftoff!"`. Hint: `(1..=from).rev()` and `push_str`.
fn countdown(from: u32) -> String {
    todo!()
}

#[test]
fn fibonacci_numbers() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(50), 12_586_269_025);
}

#[test]
fn collatz_step_counts() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(27), 111);
}

#[test]
fn odd_sums() {
    assert_eq!(sum_odds_below(1), 0);
    assert_eq!(sum_odds_below(2), 1);
    assert_eq!(sum_odds_below(10), 25);
    assert_eq!(sum_odds_below(11), 25);
}

#[test]
fn countdown_strings() {
    assert_eq!(countdown(3), "3, 2, 1, liftoff!");
    assert_eq!(countdown(1), "1, liftoff!");
    assert_eq!(countdown(0), "liftoff!");
}
