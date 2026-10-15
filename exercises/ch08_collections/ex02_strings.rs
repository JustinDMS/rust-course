//! # Strings are not arrays of chars
//!
//! Concept: no integer indexing on `String`; `len` is bytes; `chars()`
//! walks characters; `+` consumes its left operand.
//!
//! Book: 8.2 "Storing UTF-8 Encoded Text with Strings"
//!
//! Your job: two functions do not compile and one panics on non-ASCII
//! input. Fix all three so the tests pass. Do not change the tests.

/// The first character of `s`, if it has one.
fn first_char(s: &str) -> Option<char> {
    if s.is_empty() {
        None
    } else {
        Some(s[0])
    }
}

/// The first `n` characters of `s` (all of it if it is shorter).
fn take_chars(s: &str, n: usize) -> String {
    s[..n].to_string()
}

/// `first` and `last` joined with a space. The caller keeps both strings.
fn full_name(first: &String, last: &String) -> String {
    first + " " + last
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
