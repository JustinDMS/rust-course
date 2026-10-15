//! # Result with a String error
//!
//! Concept: a `Result<T, String>` is the simplest custom error: the
//! message *is* the error. `?` inside a loop stops at the first failure.
//!
//! Book: 9.2 "Recoverable Errors with Result"
//!
//! Your job: implement the three functions.

/// Parses `"3,4"` into `(3, 4)`.
///
/// Errors (exact text):
/// - no comma: `"missing comma in '<input>'"`
/// - a part that is not a number: `"bad number '<part>'"` (the part is
///   trimmed)
fn parse_pair(s: &str) -> Result<(i32, i32), String> {
    todo!()
}

/// Parses every non-empty line, stopping at the first error.
fn parse_pairs(text: &str) -> Result<Vec<(i32, i32)>, String> {
    todo!()
}

/// The sum of all `x` values and the sum of all `y` values across the
/// lines, or the first parse error.
fn sum_pairs(text: &str) -> Result<(i32, i32), String> {
    todo!()
}

#[test]
fn pairs_parse() {
    assert_eq!(parse_pair("3,4"), Ok((3, 4)));
    assert_eq!(parse_pair(" -1 , 20 "), Ok((-1, 20)));
}

#[test]
fn pair_errors_are_descriptive() {
    assert_eq!(parse_pair("34"), Err(String::from("missing comma in '34'")));
    assert_eq!(parse_pair("3,x"), Err(String::from("bad number 'x'")));
    assert_eq!(parse_pair("a, 4"), Err(String::from("bad number 'a'")));
}

#[test]
fn many_pairs() {
    assert_eq!(parse_pairs("1,2\n\n3,4\n"), Ok(vec![(1, 2), (3, 4)]));
    assert_eq!(parse_pairs(""), Ok(vec![]));
    assert_eq!(parse_pairs("1,2\n3;4"), Err(String::from("missing comma in '3;4'")));
}

#[test]
fn sums() {
    assert_eq!(sum_pairs("1,2\n3,4\n10,20"), Ok((14, 26)));
    assert_eq!(sum_pairs(""), Ok((0, 0)));
    assert_eq!(sum_pairs("1,2\nq,4"), Err(String::from("bad number 'q'")));
}
