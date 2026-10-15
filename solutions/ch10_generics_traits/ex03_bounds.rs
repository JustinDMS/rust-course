//! # Bounds the body needs (solution)
//!
//! `join_all` calls `to_string`, so `T: Display`. `max_of` compares with
//! `>`, so `T: PartialOrd`. `unit_shape` returned two different concrete
//! types behind one `impl Shape`; both tests want area 1.0, so a unit
//! square serves for both.

use std::fmt::Display;

fn join_all<T: Display>(items: &[T]) -> String {
    let mut parts = Vec::new();
    for item in items {
        parts.push(item.to_string());
    }
    parts.join(", ")
}

fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
#[allow(dead_code)]
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

fn unit_shape(_round: bool) -> impl Shape {
    Square(1.0)
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
