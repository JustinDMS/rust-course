//! # Scalars, tuples and arrays (solution)

/// Celsius to Fahrenheit.
fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Fahrenheit to Celsius.
fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// The average of four integers as a float.
fn average(values: [i32; 4]) -> f64 {
    let mut sum = 0;
    for v in values {
        sum += v;
    }
    sum as f64 / 4.0
}

/// Swaps the two elements of a pair and converts the second to a float.
fn swap_pair(pair: (i32, i32)) -> (f64, i32) {
    let (a, b) = pair;
    (b as f64, a)
}

/// `a + b` as a `u8`, or `None` if the sum would not fit.
fn add_u8(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
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
