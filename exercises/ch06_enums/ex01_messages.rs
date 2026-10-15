//! # An enum with data
//!
//! Concept: each variant of an enum can carry its own data, and `match`
//! takes it back out.
//!
//! Book: 6.1 "Defining an Enum"
//!
//! Your job: define the `Message` enum with these variants, then implement
//! `describe` and `is_noop`.
//!
//! - `Quit` (no data)
//! - `Move { x: i32, y: i32 }` (named fields)
//! - `Write(String)` (one value)
//! - `ChangeColor(u8, u8, u8)` (three values)

// TODO: define `enum Message` here. Derive `Debug` so the tests can print it.

impl Message {
    /// A human description:
    ///
    /// - `Quit`                  → `"quit"`
    /// - `Move { x: 1, y: -2 }`  → `"move to (1, -2)"`
    /// - `Write("hi")`           → `"write 'hi'"`
    /// - `ChangeColor(0, 10, 255)` → `"color #000aff"` (two lowercase hex digits each)
    fn describe(&self) -> String {
        todo!()
    }

    /// True for a `Move` that goes nowhere or a `Write` of an empty string.
    fn is_noop(&self) -> bool {
        todo!()
    }
}

#[test]
fn descriptions() {
    assert_eq!(Message::Quit.describe(), "quit");
    assert_eq!(Message::Move { x: 1, y: -2 }.describe(), "move to (1, -2)");
    assert_eq!(Message::Write(String::from("hi")).describe(), "write 'hi'");
    assert_eq!(Message::ChangeColor(0, 10, 255).describe(), "color #000aff");
}

#[test]
fn noops() {
    assert!(Message::Move { x: 0, y: 0 }.is_noop());
    assert!(Message::Write(String::new()).is_noop());
    assert!(!Message::Move { x: 0, y: 1 }.is_noop());
    assert!(!Message::Write(String::from("x")).is_noop());
    assert!(!Message::Quit.is_noop());
    assert!(!Message::ChangeColor(0, 0, 0).is_noop());
}

#[test]
fn debug_is_derived() {
    assert_eq!(format!("{:?}", Message::Quit), "Quit");
    assert_eq!(format!("{:?}", Message::Move { x: 3, y: 4 }), "Move { x: 3, y: 4 }");
}
