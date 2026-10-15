//! # Iterator adaptors (solution)

fn squares_of_evens(values: &[i32]) -> Vec<i32> {
    values.iter().filter(|&&v| v % 2 == 0).map(|v| v * v).collect()
}

fn total_len(words: &[&str]) -> usize {
    words.iter().map(|w| w.len()).sum()
}

fn longest_word(text: &str) -> Option<&str> {
    text.split_whitespace()
        .fold(None, |best: Option<&str>, word| match best {
            Some(b) if b.len() >= word.len() => Some(b),
            _ => Some(word),
        })
}

fn first_negative_index(values: &[i32]) -> Option<usize> {
    values.iter().position(|&v| v < 0)
}

fn numbered_upper(words: &[&str]) -> String {
    words
        .iter()
        .enumerate()
        .map(|(i, w)| format!("{}:{}", i + 1, w.to_uppercase()))
        .collect::<Vec<_>>()
        .join(", ")
}

#[test]
fn squares() {
    assert_eq!(squares_of_evens(&[1, 2, 3, 4]), vec![4, 16]);
    assert!(squares_of_evens(&[1, 3]).is_empty());
}

#[test]
fn lengths() {
    assert_eq!(total_len(&["ab", "cde", ""]), 5);
    assert_eq!(total_len(&[]), 0);
}

#[test]
fn longest() {
    assert_eq!(longest_word("the quick brown fox"), Some("quick"));
    assert_eq!(longest_word("aa bb cc"), Some("aa"));
    assert_eq!(longest_word("   "), None);
}

#[test]
fn negatives() {
    assert_eq!(first_negative_index(&[3, 0, -2, -5]), Some(2));
    assert_eq!(first_negative_index(&[1, 2]), None);
}

#[test]
fn numbering() {
    assert_eq!(numbered_upper(&["a", "b"]), "1:A, 2:B");
    assert_eq!(numbered_upper(&[]), "");
}
