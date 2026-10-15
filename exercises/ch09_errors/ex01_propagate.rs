//! # Propagate, don't panic
//!
//! Concept: a function that returns `Result` should hand failures to its
//! caller with `?`, not `unwrap()` them away. `Option` has helpers to turn
//! a missing value into an error (`ok_or`), and `Result` has `map_err` to
//! change the error type.
//!
//! Book: 9.2 "Recoverable Errors with Result"
//!
//! Your job: all three functions compile and have the right signature, but
//! they panic on bad input instead of returning `Err`. Fix them with `?`
//! (and `ok_or` / `map_err` where needed). The checker rejects `unwrap()`
//! and `expect(` in this file.

use std::num::ParseIntError;

/// Parses one number, ignoring surrounding whitespace.
fn read_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse().unwrap();
    Ok(n)
}

/// Sums one number per line.
fn sum_lines(text: &str) -> Result<i32, ParseIntError> {
    let mut total = 0;
    for line in text.lines() {
        total += read_number(line).unwrap();
    }
    Ok(total)
}

/// The number on the first line. Errors are reported as text:
/// `"empty input"` when there is no first line, otherwise the parse
/// error's own message (`e.to_string()`).
fn first_line_number(text: &str) -> Result<i32, String> {
    let line = text.lines().next().unwrap();
    let n = read_number(line).unwrap();
    Ok(n)
}

#[test]
fn read_number_reports_errors() {
    assert_eq!(read_number(" 42 "), Ok(42));
    assert!(read_number("forty").is_err());
}

#[test]
fn sum_lines_stops_at_the_first_bad_line() {
    assert_eq!(sum_lines("1\n2\n3"), Ok(6));
    assert_eq!(sum_lines(""), Ok(0));
    assert!(sum_lines("1\nx\n3").is_err());
}

#[test]
fn first_line_number_describes_the_problem() {
    assert_eq!(first_line_number("7\n8"), Ok(7));
    assert_eq!(first_line_number(""), Err(String::from("empty input")));
    assert_eq!(
        first_line_number("abc"),
        Err(String::from("invalid digit found in string"))
    );
}
