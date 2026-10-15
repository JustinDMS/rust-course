//! # Generic functions and structs (solution)

fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest = list.first()?;
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    largest(list).copied()
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn swapped(self) -> Point<T> {
        Point {
            x: self.y,
            y: self.x,
        }
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug, PartialEq)]
struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    fn mixup<C, D>(self, other: Pair<C, D>) -> Pair<A, D> {
        Pair {
            first: self.first,
            second: other.second,
        }
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
