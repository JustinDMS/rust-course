//! # Strings are not arrays of chars (solution)
//!
//! `s[0]` does not exist; `chars().next()` gives the first character.
//! `&s[..n]` slices bytes and panics inside a multi-byte character; take
//! `n` *characters* with `chars().take(n)` and collect them. `+` needs an
//! owned `String` on the left, so clone (or use `format!`).

/// The first character of `s`, if it has one.
fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

/// The first `n` characters of `s`.
fn take_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// `first` and `last` joined with a space.
fn full_name(first: &String, last: &String) -> String {
    format!("{first} {last}")
}

#[test]
fn first_chars() {
    assert_eq!(first_char("hello"), Some('h'));
    assert_eq!(first_char("Здравствуйте"), Some('З'));
    assert_eq!(first_char(""), None);
}

#[test]
fn taking_characters_not_bytes() {
    assert_eq!(take_chars("hello", 2), "he");
    assert_eq!(take_chars("hi", 10), "hi");
    assert_eq!(take_chars("Здравствуйте", 3), "Здр");
    assert_eq!(take_chars("héllo", 2), "hé");
}

#[test]
fn full_names_keep_their_parts() {
    let first = String::from("Samus");
    let last = String::from("Aran");
    assert_eq!(full_name(&first, &last), "Samus Aran");
    assert_eq!(first, "Samus");
    assert_eq!(last, "Aran");
}
