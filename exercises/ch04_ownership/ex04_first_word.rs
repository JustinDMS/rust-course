//! # String slices
//!
//! Concept: a `&str` is a *view* into string data: a pointer plus a length.
//! `&s[a..b]` borrows the bytes from index `a` up to (not including) `b`.
//! Returning a slice ties its lifetime to the input, so the compiler stops
//! anyone from using it after the string is gone.
//!
//! Book: 4.3 "The Slice Type"
//!
//! Your job: implement the four functions using indexing and slicing. The
//! point is to practise slices, so this exercise rejects the `split`,
//! `strip_` and `trim` helpers: find the index yourself (`find`, or
//! `bytes().enumerate()`), then slice.

/// The first word of `s`: everything before the first space, or all of `s`
/// if it has no space.
fn first_word(s: &str) -> &str {
    todo!()
}

/// Everything after the first word and its following space.
/// `"hello big world"` gives `"big world"`; `"hello"` gives `""`.
fn rest(s: &str) -> &str {
    todo!()
}

/// `s` without one pair of surrounding double quotes, if it has them.
/// `"\"beam\""` gives `"beam"`; `"beam"` gives `"beam"`; `"\""` gives `"\""`.
fn unquote(s: &str) -> &str {
    todo!()
}

/// The middle third of `s` by byte length (ASCII input only): the slice from
/// `len / 3` to `2 * len / 3`. `"abcdefghi"` gives `"def"`.
fn middle_third(s: &str) -> &str {
    todo!()
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
