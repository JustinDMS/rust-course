//! # Loops and conditions (solution)

/// The n-th Fibonacci number.
fn fib(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

/// Number of steps the Collatz sequence takes to reach 1 from `n`.
fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

/// The sum of all odd numbers strictly below `limit`.
fn sum_odds_below(limit: u32) -> u32 {
    let mut sum = 0;
    for i in 0..limit {
        if i % 2 == 1 {
            sum += i;
        }
    }
    sum
}

/// A countdown string.
fn countdown(from: u32) -> String {
    let mut text = String::new();
    for i in (1..=from).rev() {
        text.push_str(&format!("{i}, "));
    }
    text.push_str("liftoff!");
    text
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
