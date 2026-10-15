//! # String slices (solution)
//!
//! Each function finds a byte index and slices. Because the return type is
//! `&str` with one input reference, the compiler infers that the result
//! borrows from `s` (lifetime elision, chapter 10).

/// The first word of `s`: everything before the first space, or all of `s`
/// if it has no space.
fn first_word(s: &str) -> &str {
    for (i, b) in s.bytes().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Everything after the first word and its following space.
/// `"hello big world"` gives `"big world"`; `"hello"` gives `""`.
fn rest(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[i + 1..],
        None => "",
    }
}

/// `s` without one pair of surrounding double quotes, if it has them.
fn unquote(s: &str) -> &str {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// The middle third of `s` by byte length (ASCII input only).
fn middle_third(s: &str) -> &str {
    let len = s.len();
    &s[len / 3..2 * len / 3]
}

#[test]
fn first_word_stops_at_space() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("hello"), "hello");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word(" leading"), "");
}

#[test]
fn rest_skips_first_word() {
    assert_eq!(rest("hello big world"), "big world");
    assert_eq!(rest("hello"), "");
    assert_eq!(rest(""), "");
}

#[test]
fn unquote_removes_one_pair() {
    assert_eq!(unquote("\"beam\""), "beam");
    assert_eq!(unquote("beam"), "beam");
    assert_eq!(unquote("\""), "\"");
    assert_eq!(unquote("\"\""), "");
    assert_eq!(unquote("\"\"\""), "\"");
}

#[test]
fn middle_third_by_bytes() {
    assert_eq!(middle_third("abcdefghi"), "def");
    assert_eq!(middle_third("abcdef"), "cd");
    assert_eq!(middle_third(""), "");
}

#[test]
fn slices_borrow_from_the_input() {
    let owned = String::from("samus aran");
    let word = first_word(&owned);
    assert_eq!(word, "samus");
    // `word` points into `owned`: same address, no copy.
    assert_eq!(word.as_ptr(), owned.as_ptr());
}
