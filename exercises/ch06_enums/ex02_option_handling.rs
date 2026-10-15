//! # Handling Option properly
//!
//! Concept: an `Option<T>` is not a `T`. Take the value out with `match`,
//! `if let`, `let else` or a helper method, and decide what `None` means
//! every time.
//!
//! Book: 6.1 "The Option Enum and Its Advantages Over Null Values"
//!
//! Your job: three functions misuse `Option`. One does not compile, one
//! panics on valid input, one has the wrong result. Fix them so the tests
//! pass **without using `unwrap()` or `expect()` anywhere**.

/// The player's level plus one, or 1 if no level is recorded.
fn next_level(current: Option<u32>) -> u32 {
    current + 1
}

/// The display name: the nickname if there is one, otherwise the full name.
fn display_name(full_name: &str, nickname: Option<&str>) -> String {
    nickname.unwrap().to_string()
}

/// The sum of two optional scores. A missing score counts as zero, but if
/// **both** are missing the result is `None`.
fn total_score(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
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
