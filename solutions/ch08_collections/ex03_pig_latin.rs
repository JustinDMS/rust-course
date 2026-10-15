//! # Pig latin (solution)

fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    if "aeiouAEIOU".contains(first) {
        format!("{word}-hay")
    } else {
        let rest: String = chars.collect();
        format!("{rest}-{first}ay")
    }
}

fn pig_latin_sentence(sentence: &str) -> String {
    let mut words = Vec::new();
    for word in sentence.split_whitespace() {
        words.push(pig_latin(word));
    }
    words.join(" ")
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
