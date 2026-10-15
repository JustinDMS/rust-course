//! Stage 1: parsing input. Edit `projects/guessing_game/src/lib.rs`.

use guessing_game::parse_guess;

#[test]
fn plain_numbers_parse() {
    assert_eq!(parse_guess("42"), Some(42));
    assert_eq!(parse_guess("0"), Some(0));
    assert_eq!(parse_guess("100"), Some(100));
}

#[test]
fn whitespace_and_newlines_are_ignored() {
    assert_eq!(parse_guess("42\n"), Some(42));
    assert_eq!(parse_guess("  7  \r\n"), Some(7));
}

#[test]
fn garbage_is_none() {
    assert_eq!(parse_guess("forty-two"), None);
    assert_eq!(parse_guess(""), None);
    assert_eq!(parse_guess("4 2"), None);
    assert_eq!(parse_guess("-3"), None);
    assert_eq!(parse_guess("3.5"), None);
}
