//! # Moves and copies
//!
//! Concept: assigning a `String` (or passing it to a function) *moves* it.
//! The old name is dead afterwards. Types that are `Copy` (integers, bools,
//! chars, floats, shared references, tuples of Copy types) are duplicated
//! instead, and the old name stays usable.
//!
//! Book: 4.1 "What Is Ownership?"
//!
//! Your job: make the tests at the bottom compile and pass. Do not change the
//! tests. There are two separate problems in this file.

/// Returns `name` in upper case.
fn make_upper(name: String) -> String {
    name.to_uppercase()
}

/// Describes a player as `"<UPPER NAME> (<length> letters)"`.
fn describe(name: String) -> String {
    let upper = make_upper(name);
    let letters = name.len();
    format!("{upper} ({letters} letters)")
}

/// A tag attached to an entity. Many entities share the same tag text.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Tag {
    text: String,
}

/// Gives every one of `count` entities the same tag.
fn tag_all(tag: Tag, count: usize) -> Vec<Tag> {
    let mut tags = Vec::new();
    for _ in 0..count {
        tags.push(tag);
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
