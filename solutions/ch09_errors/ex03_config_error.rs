//! # A custom error type (solution)

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ConfigError {
    Missing(String),
    BadNumber(ParseIntError),
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

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

#[derive(Debug, PartialEq)]
struct Config {
    width: u32,
    height: u32,
}

fn parse_config(text: &str) -> Result<Config, ConfigError> {
    let mut width = None;
    let mut height = None;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(ConfigError::Malformed(line.to_string()));
        };
        match key.trim() {
            "width" => width = Some(value.trim().parse()?),
            "height" => height = Some(value.trim().parse()?),
            _ => {}
        }
    }
    let width = width.ok_or(ConfigError::Missing(String::from("width")))?;
    let height = height.ok_or(ConfigError::Missing(String::from("height")))?;
    Ok(Config { width, height })
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
