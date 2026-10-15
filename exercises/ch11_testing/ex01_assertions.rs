//! # Fix the tests, not the code
//!
//! Concept: `assert!`, `assert_eq!`, `assert_ne!` and custom failure
//! messages. For once, the functions are correct and the **tests** are
//! wrong.
//!
//! Book: 11.1 "How to Write Tests"
//!
//! Your job: read each failing test's output, decide what the test
//! *meant* to check, and correct the test. Do not change the three
//! functions.

/// `a + b`
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// True for even numbers.
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// `"Hello, <name>!"`
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[test]
fn adding() {
    assert_eq!(add(2, 2), 5);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn evenness() {
    assert!(is_even(3));
    assert!(!is_even(4));
}

#[test]
fn greeting_contains_the_name() {
    let result = greet("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
    );
    assert_ne!(result, "Hello, Carol!");
}
