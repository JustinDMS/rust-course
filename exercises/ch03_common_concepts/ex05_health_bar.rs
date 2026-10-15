//! # A health bar
//!
//! Concept: integer arithmetic truncates, `if`/`else if` chains produce a
//! value, and `str::repeat` builds repeated text.
//!
//! Book: 3.2 "Data Types" and 3.5 "Control Flow"
//!
//! A HUD element shows a player's health as a bar:
//!
//! ```text
//! [#####-----] 50%
//! ```
//!
//! Your job: implement `percent`, `health_bar` and `status`.

/// Health as a whole-number percentage, rounded down: 1/3 is 33.
/// `hp` above `max` counts as full. `max` is never 0.
fn percent(hp: u32, max: u32) -> u32 {
    todo!()
}

/// `[` + `width` characters + `] NN%`, where the first
/// `width * hp / max` characters are `#` and the rest `-`.
/// `health_bar(50, 100, 10)` is `"[#####-----] 50%"`.
fn health_bar(hp: u32, max: u32, width: usize) -> String {
    todo!()
}

/// `"full"` at 100%, `"hurt"` at 50% to 99%, `"wounded"` at 25% to 49%,
/// `"critical"` below 25%.
fn status(hp: u32, max: u32) -> &'static str {
    todo!()
}

#[test]
fn percent_rounds_down() {
    assert_eq!(percent(1, 3), 33);
    assert_eq!(percent(2, 3), 66);
    assert_eq!(percent(0, 99), 0);
    assert_eq!(percent(99, 99), 100);
    assert_eq!(percent(150, 99), 100);
}

#[test]
fn bars() {
    assert_eq!(health_bar(50, 100, 10), "[#####-----] 50%");
    assert_eq!(health_bar(100, 100, 4), "[####] 100%");
    assert_eq!(health_bar(0, 100, 4), "[----] 0%");
    assert_eq!(health_bar(33, 99, 6), "[##----] 33%");
    assert_eq!(health_bar(1, 100, 10), "[----------] 1%");
}

#[test]
fn statuses() {
    assert_eq!(status(99, 99), "full");
    assert_eq!(status(98, 99), "hurt");
    assert_eq!(status(50, 100), "hurt");
    assert_eq!(status(49, 100), "wounded");
    assert_eq!(status(25, 100), "wounded");
    assert_eq!(status(24, 100), "critical");
    assert_eq!(status(0, 100), "critical");
}
