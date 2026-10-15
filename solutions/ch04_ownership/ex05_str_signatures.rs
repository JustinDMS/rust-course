//! # `&str` over `&String` (solution)
//!
//! Read-only string parameters become `&str`; the one function that mutates
//! takes `&mut String`. Nothing else changes.

/// True if `s` has at least one letter and no lower case letters.
fn is_shout(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic()) && !s.chars().any(|c| c.is_lowercase())
}

/// Number of whitespace-separated words.
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Appends `!` to the caller's string.
fn exclaim(s: &mut String) {
    s.push('!');
}

/// Joins a name and a title: `("Samus", "Hunter")` gives `"Samus the Hunter"`.
fn with_title(name: &str, title: &str) -> String {
    format!("{name} the {title}")
}

#[test]
fn literals_are_slices() {
    assert!(is_shout("HEY"));
    assert!(!is_shout("Hey"));
    assert!(!is_shout("123"));
    assert_eq!(word_count("one two  three"), 3);
}

#[test]
fn owned_strings_coerce() {
    let owned = String::from("MISSILES READY");
    assert!(is_shout(&owned));
    assert_eq!(word_count(&owned), 2);
    assert_eq!(owned, "MISSILES READY");
}

#[test]
fn exclaim_changes_the_caller_string() {
    let mut s = String::from("hi");
    exclaim(&mut s);
    exclaim(&mut s);
    assert_eq!(s, "hi!!");
}

#[test]
fn with_title_accepts_any_mix() {
    let name = String::from("Samus");
    assert_eq!(with_title(&name, "Hunter"), "Samus the Hunter");
    assert_eq!(with_title("Ridley", &String::from("Dragon")), "Ridley the Dragon");
    assert_eq!(with_title(&name[..3], "Sam"), "Sam the Sam");
}
