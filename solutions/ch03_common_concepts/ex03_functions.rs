//! # Statements and expressions (solution)
//!
//! `square`: drop the semicolon so `x * x` is the value of the block.
//! `max3`: every parameter needs a type.
//! `double_plus_one`: `let` is a statement; just compute the value.

/// `x * x`
fn square(x: i32) -> i32 {
    x * x
}

/// The larger of three numbers.
fn max3(a: i32, b: i32, c: i32) -> i32 {
    let mut best = a;
    if b > best {
        best = b;
    }
    if c > best {
        best = c;
    }
    best
}

/// Doubles `n`, then adds one.
fn double_plus_one(n: i32) -> i32 {
    let doubled = n * 2;
    doubled + 1
}

#[test]
fn square_squares() {
    assert_eq!(square(0), 0);
    assert_eq!(square(7), 49);
    assert_eq!(square(-3), 9);
}

#[test]
fn max3_finds_the_largest() {
    assert_eq!(max3(1, 2, 3), 3);
    assert_eq!(max3(9, 2, 3), 9);
    assert_eq!(max3(1, 8, 3), 8);
    assert_eq!(max3(-1, -2, -3), -1);
}

#[test]
fn double_plus_one_works() {
    assert_eq!(double_plus_one(0), 1);
    assert_eq!(double_plus_one(20), 41);
}
