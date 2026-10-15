//! # Scalars, tuples and arrays
//!
//! Concept: numeric types never convert implicitly (`as` does it
//! explicitly), integer overflow is a bug Rust makes you decide about, and
//! tuples and arrays are fixed-size compound types.
//!
//! Book: 3.2 "Data Types"
//!
//! Your job: implement the five functions.

/// Celsius to Fahrenheit: `c * 9 / 5 + 32`.
fn to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

/// Fahrenheit to Celsius: `(f - 32) * 5 / 9`.
fn to_celsius(fahrenheit: f64) -> f64 {
    todo!()
}

/// The average of four integers as a float: `[1, 2, 3, 4]` gives `2.5`.
/// Hint: sum as an integer, then convert with `as f64` before dividing.
fn average(values: [i32; 4]) -> f64 {
    todo!()
}

/// Swaps the two elements of a pair and converts the second to a float:
/// `(3, 4)` gives `(4.0, 3)`.
fn swap_pair(pair: (i32, i32)) -> (f64, i32) {
    todo!()
}

/// `a + b` as a `u8`, or `None` if the sum would not fit.
/// Hint: integers have a `checked_add` method.
fn add_u8(a: u8, b: u8) -> Option<u8> {
    todo!()
}

#[test]
fn temperature_conversions() {
    assert_eq!(to_fahrenheit(0.0), 32.0);
    assert_eq!(to_fahrenheit(100.0), 212.0);
    assert_eq!(to_celsius(212.0), 100.0);
    assert!((to_celsius(98.6) - 37.0).abs() < 0.01);
}

#[test]
fn average_is_a_float() {
    assert_eq!(average([1, 2, 3, 4]), 2.5);
    assert_eq!(average([10, 10, 10, 10]), 10.0);
    assert_eq!(average([-1, 1, -1, 1]), 0.0);
}

#[test]
fn swap_pair_swaps_and_converts() {
    assert_eq!(swap_pair((3, 4)), (4.0, 3));
    assert_eq!(swap_pair((0, -7)), (-7.0, 0));
}

#[test]
fn add_u8_detects_overflow() {
    assert_eq!(add_u8(100, 100), Some(200));
    assert_eq!(add_u8(255, 0), Some(255));
    assert_eq!(add_u8(200, 100), None);
}
