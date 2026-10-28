//! # Your first functions
//!
//! Concept: `format!` builds a `String` from a format string, exactly like
//! `println!` prints one. `str::repeat(n)` repeats a piece of text.
//!
//! Book: 1.2 "Hello, World!" and 1.3 "Hello, Cargo!"
//!
//! Your job: replace each `todo!()` so the tests pass. Run them with the
//! *Run tests* button (or Ctrl+Enter in the editor).

/// `"Hello, <name>!"`
fn greeting(name: &str) -> String {
    todo!()
}

/// `text` on its own line between two lines of `=`, each as long as `text`:
///
/// ```text
/// =====
/// Hello
/// =====
/// ```
///
/// (No trailing newline after the last line.)
fn banner(text: &str) -> String {
    todo!()
}

/// `n` copies of `"Hello! "` with the trailing space removed:
/// `shout(3)` gives `"Hello! Hello! Hello!"`, and `shout(0)` gives `""`.
fn shout(n: usize) -> String {
    todo!()
}

#[test]
fn greeting_mentions_the_name() {
    assert_eq!(greeting("world"), "Hello, world!");
    assert_eq!(greeting("Samus"), "Hello, Samus!");
}

#[test]
fn banner_wraps_text_in_equals_signs() {
    assert_eq!(banner("Hello"), "=====\nHello\n=====");
    assert_eq!(banner("hi"), "==\nhi\n==");
}

#[test]
fn shout_repeats() {
    assert_eq!(shout(1), "Hello!");
    assert_eq!(shout(3), "Hello! Hello! Hello!");
    assert_eq!(shout(0), "");
}
