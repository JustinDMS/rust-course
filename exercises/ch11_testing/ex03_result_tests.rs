//! # Tests that return Result
//!
//! Concept: a test can return `Result<(), E>` so that `?` works inside it.
//! An `Err` fails the test.
//!
//! Book: 11.1 "Using Result<T, E> in Tests"
//!
//! Your job: implement `parse_version` so the `Result`-returning tests
//! pass, then write **one more** test of your own, also returning
//! `Result<(), String>`, that checks `"0.0.0"` parses to `(0, 0, 0)`.

/// Parses `"1.2.3"` into `(1, 2, 3)`.
///
/// Errors: `"expected 3 parts, got <n>"` for the wrong number of
/// dot-separated parts; `"bad number '<part>'"` for a part that is not a
/// `u32`.
fn parse_version(s: &str) -> Result<(u32, u32, u32), String> {
    todo!()
}

#[test]
fn parses_a_normal_version() -> Result<(), String> {
    let v = parse_version("1.2.3")?;
    assert_eq!(v, (1, 2, 3));
    Ok(())
}

#[test]
fn parses_big_numbers() -> Result<(), String> {
    let (major, minor, patch) = parse_version("10.200.3000")?;
    assert_eq!(major + minor + patch, 3210);
    Ok(())
}

#[test]
fn wrong_part_count_is_an_error() {
    assert_eq!(parse_version("1.2"), Err(String::from("expected 3 parts, got 2")));
    assert_eq!(parse_version("1.2.3.4"), Err(String::from("expected 3 parts, got 4")));
}

#[test]
fn bad_numbers_are_errors() {
    assert_eq!(parse_version("1.x.3"), Err(String::from("bad number 'x'")));
}

// TODO: add a test named `parses_zero` that returns Result<(), String>.
