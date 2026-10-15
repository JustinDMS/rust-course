//! # Your first functions (solution)

/// `"Hello, <name>!"`
fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

/// `text` on its own line between two lines of `=`.
fn banner(text: &str) -> String {
    let line = "=".repeat(text.len());
    format!("{line}\n{text}\n{line}")
}

/// `n` copies of `"Hello! "` with the trailing space removed.
fn shout(n: usize) -> String {
    "Hello! ".repeat(n).trim_end().to_string()
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
