//! # Moves and copies (solution)
//!
//! Problem 1: `describe` moved `name` into `make_upper`, then used it.
//! Measure the length first (a `usize` is `Copy`), or borrow instead.
//!
//! Problem 2: `Tag` holds a `String`, which owns heap memory, so it cannot be
//! `Copy`. Drop the derive and clone explicitly inside the loop.

/// Returns `name` in upper case.
fn make_upper(name: String) -> String {
    name.to_uppercase()
}

/// Describes a player as `"<UPPER NAME> (<length> letters)"`.
fn describe(name: String) -> String {
    let letters = name.len();
    let upper = make_upper(name);
    format!("{upper} ({letters} letters)")
}

/// A tag attached to an entity. Many entities share the same tag text.
#[derive(Debug, Clone, PartialEq)]
struct Tag {
    text: String,
}

/// Gives every one of `count` entities the same tag.
fn tag_all(tag: Tag, count: usize) -> Vec<Tag> {
    let mut tags = Vec::new();
    for _ in 0..count {
        tags.push(tag.clone());
    }
    tags
}

#[test]
fn describe_mentions_upper_name_and_length() {
    assert_eq!(describe(String::from("Samus")), "SAMUS (5 letters)");
}

#[test]
fn describe_handles_empty_names() {
    assert_eq!(describe(String::new()), " (0 letters)");
}

#[test]
fn tag_all_hands_out_equal_tags() {
    let tag = Tag {
        text: String::from("hostile"),
    };
    let tags = tag_all(tag.clone(), 3);
    assert_eq!(tags.len(), 3);
    assert!(tags.iter().all(|t| *t == tag));
}
