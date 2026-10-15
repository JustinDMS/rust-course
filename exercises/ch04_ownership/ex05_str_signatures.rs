//! # `&str` over `&String`
//!
//! Concept: a parameter typed `&str` accepts a string literal, a `&String`
//! (through deref coercion) and a slice of either. A parameter typed
//! `&String` accepts only a `&String`. Take `String` by value only when you
//! need to own it; take `&mut String` when you need to change the caller's
//! string in place.
//!
//! Book: 4.3 "The Slice Type" (String Slices as Parameters)
//!
//! Your job: the function *bodies* are fine; the parameter types are not.
//! Change the signatures so the tests compile. Do not change the tests.

/// True if `s` has at least one letter and no lower case letters.
fn is_shout(s: &String) -> bool {
    s.chars().any(|c| c.is_alphabetic()) && !s.chars().any(|c| c.is_lowercase())
}

/// Number of whitespace-separated words.
fn word_count(s: String) -> usize {
    s.split_whitespace().count()
}

/// Appends `!` to the caller's string.
fn exclaim(s: String) {
    s.push('!');
}

/// Joins a name and a title: `("Samus", "Hunter")` gives `"Samus the Hunter"`.
fn with_title(name: &String, title: &String) -> String {
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
