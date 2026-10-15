//! # Methods and associated functions (solution)

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn rotated(self) -> Rectangle {
        Rectangle {
            width: self.height,
            height: self.width,
        }
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
