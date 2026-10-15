//! # Loading save slots
//!
//! Concept: a parser with a typed error, composed with `?`, and a loader
//! that boxes any error for a caller who only wants to print it.
//!
//! Book: Chapter 9 (all of it)
//!
//! A save file has one slot per line: `name=Samus;hp=99;level=3`. Fields
//! may come in any order; all three are required. Blank lines are skipped.
//!
//! Your job: implement `parse_slot` and `load_slots`.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct SaveSlot {
    name: String,
    hp: u32,
    level: u32,
}

#[derive(Debug, PartialEq)]
enum SaveError {
    /// A `key=value` piece without `=`.
    Malformed(String),
    /// A required field was not present.
    MissingField(&'static str),
    /// `hp` or `level` was not a number.
    BadNumber(ParseIntError),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::Malformed(piece) => write!(f, "malformed piece '{piece}'"),
            SaveError::MissingField(field) => write!(f, "missing field '{field}'"),
            SaveError::BadNumber(e) => write!(f, "bad number: {e}"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<ParseIntError> for SaveError {
    fn from(e: ParseIntError) -> Self {
        SaveError::BadNumber(e)
    }
}

/// Parses one line. Pieces are separated by `;`, each piece is `key=value`.
/// Unknown keys are ignored.
fn parse_slot(line: &str) -> Result<SaveSlot, SaveError> {
    todo!()
}

/// Parses every non-blank line. Any error is boxed so the caller can just
/// print it.
fn load_slots(text: &str) -> Result<Vec<SaveSlot>, Box<dyn std::error::Error>> {
    todo!()
}

#[test]
fn slots_parse() {
    assert_eq!(
        parse_slot("name=Samus;hp=99;level=3"),
        Ok(SaveSlot { name: String::from("Samus"), hp: 99, level: 3 })
    );
    assert_eq!(
        parse_slot("level=1;name=Ridley;ship=gunship;hp=5"),
        Ok(SaveSlot { name: String::from("Ridley"), hp: 5, level: 1 })
    );
}

#[test]
fn slot_errors() {
    assert_eq!(parse_slot("name=Samus;hp=99"), Err(SaveError::MissingField("level")));
    assert_eq!(parse_slot("hp=1;level=2"), Err(SaveError::MissingField("name")));
    assert_eq!(parse_slot("name=Samus;hp;level=2"), Err(SaveError::Malformed(String::from("hp"))));
    assert!(matches!(parse_slot("name=S;hp=full;level=2"), Err(SaveError::BadNumber(_))));
}

#[test]
fn loading_many() {
    let slots = load_slots("name=A;hp=1;level=1\n\nname=B;hp=2;level=2\n").unwrap();
    assert_eq!(slots.len(), 2);
    assert_eq!(slots[1].name, "B");
    assert!(load_slots("").unwrap().is_empty());
}

#[test]
fn loading_reports_the_first_error_as_text() {
    let err = load_slots("name=A;hp=1;level=1\nname=B;hp=x;level=2").unwrap_err();
    assert_eq!(err.to_string(), "bad number: invalid digit found in string");
    let err = load_slots("name=A").unwrap_err();
    assert_eq!(err.to_string(), "missing field 'hp'");
}
