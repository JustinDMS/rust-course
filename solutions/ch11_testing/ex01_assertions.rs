//! # Fix the tests, not the code (solution)
//!
//! `add(2, 2)` is 4; 3 is odd and 4 is even; the message was missing its
//! argument; and the greeting *is* `"Hello, Carol!"`, so `assert_eq!`.

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
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn evenness() {
    assert!(!is_even(3));
    assert!(is_even(4));
}

#[test]
fn greeting_contains_the_name() {
    let result = greet("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`",
    );
    assert_eq!(result, "Hello, Carol!");
}
