//! # Generic functions and structs
//!
//! Concept: a type parameter with the bounds the body needs; generic
//! structs with generic and type-specific `impl` blocks.
//!
//! Book: 10.1 "Generic Data Types"
//!
//! Your job: implement every `todo!()`. Add bounds to the type parameters
//! where the compiler asks for them.

/// A reference to the largest element, or `None` for an empty slice.
fn largest<T>(list: &[T]) -> Option<&T> {
    todo!()
}

/// The largest element by value; only for types that are cheap to copy.
fn largest_copy<T>(list: &[T]) -> Option<T> {
    todo!()
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        todo!()
    }

    /// Swaps the coordinates. Consumes the point.
    fn swapped(self) -> Point<T> {
        todo!()
    }
}

impl Point<f64> {
    /// Only `f64` points know their distance from the origin.
    fn distance_from_origin(&self) -> f64 {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    /// `self.first` with `other.second`. The result's type parameters come
    /// from both inputs.
    fn mixup<C, D>(self, other: Pair<C, D>) -> Pair<A, D> {
        todo!()
    }
}

#[test]
fn largest_works_for_several_types() {
    assert_eq!(largest(&[3, 9, 2]), Some(&9));
    assert_eq!(largest(&[1.5, 0.5]), Some(&1.5));
    assert_eq!(largest(&['a', 'z', 'm']), Some(&'z'));
    assert_eq!(largest(&["pear", "apple"]), Some(&"pear"));
    assert_eq!(largest::<i32>(&[]), None);
}

#[test]
fn largest_copy_returns_values() {
    assert_eq!(largest_copy(&[3, 9, 2]), Some(9));
    assert_eq!(largest_copy::<u8>(&[]), None);
}

#[test]
fn points() {
    let p = Point { x: 5, y: 10 };
    assert_eq!(*p.x(), 5);
    assert_eq!(p.swapped(), Point { x: 10, y: 5 });
    let q = Point { x: 3.0, y: 4.0 };
    assert_eq!(q.distance_from_origin(), 5.0);
}

#[test]
fn mixup_combines_pairs() {
    let a = Pair { first: 5, second: 10.4 };
    let b = Pair { first: "hello", second: 'c' };
    assert_eq!(a.mixup(b), Pair { first: 5, second: 'c' });
}
