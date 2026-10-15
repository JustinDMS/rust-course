//! # A borrowing tokenizer (solution)
//!
//! `next_token` returns `Option<&'a str>`: a slice of the *input*. If it
//! returned `Option<&str>`, elision would tie the slice to `&mut self`,
//! and holding two tokens at once (or keeping tokens after the tokenizer
//! is dropped) would be rejected.

struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, pos: 0 }
    }

    fn next_token(&mut self) -> Option<&'a str> {
        let rest = &self.input[self.pos..];
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            self.pos = self.input.len();
            return None;
        }
        let start = self.pos + (rest.len() - trimmed.len());
        let len = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        self.pos = start + len;
        Some(&self.input[start..start + len])
    }

    fn rest(&self) -> &'a str {
        &self.input[self.pos..]
    }
}

#[test]
fn tokens_in_order() {
    let mut t = Tokenizer::new("  let x =  5;");
    assert_eq!(t.next_token(), Some("let"));
    assert_eq!(t.rest(), " x =  5;");
    assert_eq!(t.next_token(), Some("x"));
    assert_eq!(t.next_token(), Some("="));
    assert_eq!(t.next_token(), Some("5;"));
    assert_eq!(t.next_token(), None);
    assert_eq!(t.next_token(), None);
    assert_eq!(t.rest(), "");
}

#[test]
fn tokens_outlive_the_tokenizer() {
    let text = String::from("alpha beta");
    let mut tokens: Vec<&str> = Vec::new();
    {
        let mut t = Tokenizer::new(&text);
        while let Some(tok) = t.next_token() {
            tokens.push(tok);
        }
    } // t is gone; the tokens still borrow from `text`
    assert_eq!(tokens, vec!["alpha", "beta"]);
}

#[test]
fn tokens_can_be_held_while_tokenizing() {
    let mut t = Tokenizer::new("a b c");
    let first = t.next_token().unwrap();
    let second = t.next_token().unwrap();
    assert_eq!((first, second), ("a", "b"));
}
