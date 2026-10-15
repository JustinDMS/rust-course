//! # A custom error type
//!
//! Concept: an error enum lets callers tell failures apart; `From` lets
//! `?` convert a library error into yours automatically; `Display` and
//! `Error` make it printable and boxable.
//!
//! Book: 9.2 and 9.3
//!
//! A config is text with `key=value` lines. `width` and `height` are
//! required and must be positive integers; other keys are ignored.
//!
//! Your job: implement `From<ParseIntError>` and `parse_config`. The
//! `Display` impl is written for you as a model of the pattern.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ConfigError {
    /// A required key was not present.
    Missing(String),
    /// A value was not a valid integer.
    BadNumber(ParseIntError),
    /// A line did not have the form `key=value`.
    Malformed(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "missing key '{key}'"),
            ConfigError::BadNumber(e) => write!(f, "bad number: {e}"),
            ConfigError::Malformed(line) => write!(f, "malformed line '{line}'"),
        }
    }
}

impl std::error::Error for ConfigError {}

// TODO: impl From<ParseIntError> for ConfigError, wrapping it in BadNumber.

#[derive(Debug, PartialEq)]
struct Config {
    width: u32,
    height: u32,
}

/// Parses the config. Blank lines are skipped. The first problem found
/// is returned; missing keys are reported only after every line parsed.
fn parse_config(text: &str) -> Result<Config, ConfigError> {
    todo!()
}

#[test]
fn good_configs() {
    assert_eq!(
        parse_config("width=800\nheight=600"),
        Ok(Config { width: 800, height: 600 })
    );
    assert_eq!(
        parse_config("title=hi\n\nheight = 10\nwidth= 20 "),
        Ok(Config { width: 20, height: 10 })
    );
}

#[test]
fn missing_keys() {
    assert_eq!(parse_config("width=800"), Err(ConfigError::Missing(String::from("height"))));
    assert_eq!(parse_config(""), Err(ConfigError::Missing(String::from("width"))));
}

#[test]
fn bad_numbers_come_through_from() {
    let err = parse_config("width=wide\nheight=600").unwrap_err();
    assert!(matches!(err, ConfigError::BadNumber(_)));
    assert_eq!(err.to_string(), "bad number: invalid digit found in string");
    let direct: ConfigError = "x".parse::<u32>().unwrap_err().into();
    assert!(matches!(direct, ConfigError::BadNumber(_)));
}

#[test]
fn malformed_lines() {
    assert_eq!(
        parse_config("width=800\nheight 600"),
        Err(ConfigError::Malformed(String::from("height 600")))
    );
}

#[test]
fn errors_can_be_boxed() {
    fn load(text: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config = parse_config(text)?;
        Ok(config)
    }
    assert_eq!(load("").unwrap_err().to_string(), "missing key 'width'");
}
