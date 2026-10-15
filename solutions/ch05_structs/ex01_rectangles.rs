//! # Rectangles, with plain functions (solution)

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// A rectangle with the given width and height.
fn rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

/// `width * height`
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

/// True if `inner` fits entirely inside `outer`.
fn can_hold(outer: &Rectangle, inner: &Rectangle) -> bool {
    outer.width > inner.width && outer.height > inner.height
}

#[test]
fn rectangles_have_fields() {
    let r = rectangle(30, 50);
    assert_eq!(r.width, 30);
    assert_eq!(r.height, 50);
    assert_eq!(r, Rectangle { width: 30, height: 50 });
}

#[test]
fn area_multiplies() {
    assert_eq!(area(&rectangle(30, 50)), 1500);
    assert_eq!(area(&rectangle(0, 50)), 0);
}

#[test]
fn can_hold_compares_both_dimensions() {
    let big = rectangle(30, 50);
    let small = rectangle(10, 40);
    let tall = rectangle(10, 60);
    assert!(can_hold(&big, &small));
    assert!(!can_hold(&small, &big));
    assert!(!can_hold(&big, &tall));
    assert!(!can_hold(&big, &big));
}

#[test]
fn debug_output_is_derived() {
    let text = format!("{:?}", rectangle(1, 2));
    assert_eq!(text, "Rectangle { width: 1, height: 2 }");
}
