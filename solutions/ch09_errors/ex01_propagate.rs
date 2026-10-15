//! # Propagate, don't panic (solution)

use std::num::ParseIntError;

/// Parses one number, ignoring surrounding whitespace.
fn read_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse()?;
    Ok(n)
}

/// Sums one number per line.
fn sum_lines(text: &str) -> Result<i32, ParseIntError> {
    let mut total = 0;
    for line in text.lines() {
        total += read_number(line)?;
    }
    Ok(total)
}

/// The number on the first line, with text errors.
fn first_line_number(text: &str) -> Result<i32, String> {
    let line = text.lines().next().ok_or(String::from("empty input"))?;
    let n = read_number(line).map_err(|e| e.to_string())?;
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
