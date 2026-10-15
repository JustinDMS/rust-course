//! # Methods and associated functions
//!
//! Concept: `impl Type { fn name(&self) ... }` attaches behaviour to a
//! type. `&self` reads, `&mut self` changes, and a function without `self`
//! is called on the type itself with `::`.
//!
//! Book: 5.3 "Method Syntax"
//!
//! Your job: fill in the `impl` block.

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// A rectangle with equal sides. Associated function: `Rectangle::square(3)`.
    fn square(size: u32) -> Self {
        todo!()
    }

    /// `width * height`
    fn area(&self) -> u32 {
        todo!()
    }

    /// `2 * (width + height)`
    fn perimeter(&self) -> u32 {
        todo!()
    }

    /// True if `other` fits strictly inside `self`.
    fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// Multiplies both sides by `factor`, in place.
    fn scale(&mut self, factor: u32) {
        todo!()
    }

    /// A new rectangle with width and height swapped. Consumes `self`.
    fn rotated(self) -> Rectangle {
        todo!()
    }
}

#[test]
fn squares_have_equal_sides() {
    assert_eq!(Rectangle::square(3), Rectangle { width: 3, height: 3 });
}

#[test]
fn area_and_perimeter() {
    let r = Rectangle { width: 3, height: 4 };
    assert_eq!(r.area(), 12);
    assert_eq!(r.perimeter(), 14);
}

#[test]
fn can_hold_uses_self() {
    let big = Rectangle { width: 8, height: 7 };
    let small = Rectangle { width: 5, height: 1 };
    assert!(big.can_hold(&small));
    assert!(!small.can_hold(&big));
}

#[test]
fn scale_changes_in_place() {
    let mut r = Rectangle { width: 2, height: 5 };
    r.scale(3);
    assert_eq!(r, Rectangle { width: 6, height: 15 });
}

#[test]
fn rotated_swaps() {
    let r = Rectangle { width: 2, height: 5 };
    assert_eq!(r.rotated(), Rectangle { width: 5, height: 2 });
}
