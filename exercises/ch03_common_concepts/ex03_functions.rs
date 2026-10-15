//! # Statements and expressions
//!
//! Concept: the last expression of a block is its value; add a semicolon
//! and it becomes a statement whose value is `()`. Parameter types are
//! never inferred. `let` is a statement and cannot be used as a value.
//!
//! Book: 3.3 "Functions"
//!
//! Your job: three functions do not compile. Make them compile and pass
//! without changing the tests.

/// `x * x`
fn square(x: i32) -> i32 {
    x * x;
}

/// The larger of three numbers.
fn max3(a: i32, b, c: i32) -> i32 {
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
    let doubled = (let d = n * 2);
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
