//! # Handling Option properly (solution)
//!
//! `next_level`: `unwrap_or(0) + 1`, or a `match`.
//! `display_name`: `unwrap` panicked on `None`; `match` or `unwrap_or`.
//! `total_score`: the arms needed to distinguish "one missing" from "both
//! missing".

/// The player's level plus one, or 1 if no level is recorded.
fn next_level(current: Option<u32>) -> u32 {
    current.unwrap_or(0) + 1
}

/// The display name: the nickname if there is one, otherwise the full name.
fn display_name(full_name: &str, nickname: Option<&str>) -> String {
    match nickname {
        Some(nick) => nick.to_string(),
        None => full_name.to_string(),
    }
}

/// The sum of two optional scores.
fn total_score(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        (Some(x), None) | (None, Some(x)) => Some(x),
        (None, None) => None,
    }
}

#[test]
fn next_level_starts_at_one() {
    assert_eq!(next_level(None), 1);
    assert_eq!(next_level(Some(4)), 5);
}

#[test]
fn display_name_prefers_the_nickname() {
    assert_eq!(display_name("Samus Aran", Some("Samus")), "Samus");
    assert_eq!(display_name("Samus Aran", None), "Samus Aran");
}

#[test]
fn total_score_treats_one_missing_as_zero() {
    assert_eq!(total_score(Some(2), Some(3)), Some(5));
    assert_eq!(total_score(Some(2), None), Some(2));
    assert_eq!(total_score(None, Some(3)), Some(3));
    assert_eq!(total_score(None, None), None);
}
