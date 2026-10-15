//! # Bounds the body needs
//!
//! Concept: a generic function may only use what its bounds promise.
//! `impl Trait` in return position must be one concrete type.
//!
//! Book: 10.2 "Traits as Parameters" and "Returning Types That Implement Traits"
//!
//! Your job: three compile errors. Fix the signatures (or, for the last
//! one, the body) without changing the tests.

use std::fmt::Display;

/// Joins the items with `", "`.
fn join_all<T>(items: &[T]) -> String {
    let mut parts = Vec::new();
    for item in items {
        parts.push(item.to_string());
    }
    parts.join(", ")
}

/// The larger of two values, by value.
fn max_of<T>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

/// A shape with area 1.0. This was meant to return a circle or a square
/// depending on `round`, but `impl Shape` can only stand for **one**
/// concrete type. Both tests expect an area of 1.0, so a unit square
/// will do in either case.
fn unit_shape(round: bool) -> impl Shape {
    if round { Circle(1.0) } else { Square(1.0) }
}

#[test]
fn joining() {
    assert_eq!(join_all(&[1, 2, 3]), "1, 2, 3");
    assert_eq!(join_all(&["a", "b"]), "a, b");
    assert_eq!(join_all::<f64>(&[]), "");
}

#[test]
fn maxing() {
    assert_eq!(max_of(3, 9), 9);
    assert_eq!(max_of(2.5, -1.0), 2.5);
    assert_eq!(max_of("b", "a"), "b");
}

#[test]
fn unit_shapes() {
    assert_eq!(unit_shape(false).area(), 1.0);
    assert_eq!(unit_shape(true).area(), 1.0);
}

#[allow(dead_code)]
fn display_check<T: Display>(_: &T) {}
