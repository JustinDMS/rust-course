//! # Pig latin
//!
//! Concept: building strings character by character, `chars()`, `push`,
//! `push_str`, `format!`, and `split_whitespace` / joining.
//!
//! Book: 8.2 (summary exercise)
//!
//! The rules:
//!
//! - A word starting with a vowel (`a e i o u`, either case) gets `-hay`
//!   appended: `apple` → `apple-hay`.
//! - Otherwise the first letter moves to the end, followed by `ay`:
//!   `first` → `irst-fay`.
//! - Upper case is kept as is; `Hello` → `ello-Hay`.
//! - An empty string stays empty.
//!
//! Your job: implement `pig_latin` for one word and `pig_latin_sentence`
//! for whitespace-separated words joined back with single spaces.

fn pig_latin(word: &str) -> String {
    todo!()
}

fn pig_latin_sentence(sentence: &str) -> String {
    todo!()
}

#[test]
fn vowel_words() {
    assert_eq!(pig_latin("apple"), "apple-hay");
    assert_eq!(pig_latin("Eat"), "Eat-hay");
    assert_eq!(pig_latin("i"), "i-hay");
}

#[test]
fn consonant_words() {
    assert_eq!(pig_latin("first"), "irst-fay");
    assert_eq!(pig_latin("Hello"), "ello-Hay");
    assert_eq!(pig_latin("x"), "-xay");
}

#[test]
fn edge_cases() {
    assert_eq!(pig_latin(""), "");
    assert_eq!(pig_latin("über"), "ber-üay");
}

#[test]
fn sentences() {
    assert_eq!(pig_latin_sentence("first apple"), "irst-fay apple-hay");
    assert_eq!(pig_latin_sentence("  spaced   out "), "paced-say out-hay");
    assert_eq!(pig_latin_sentence(""), "");
}
