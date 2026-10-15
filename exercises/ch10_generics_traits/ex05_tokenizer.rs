//! # A borrowing tokenizer
//!
//! Concept: a struct that holds a `&'a str` can hand out `&'a str` slices
//! of it. Those slices are tied to the *input*, not to the struct, so they
//! stay valid after the struct is gone and while the struct is being
//! mutated.
//!
//! Book: 10.3 "Lifetime Annotations in Struct Definitions" and "in Method
//! Definitions"
//!
//! Your job: fill in the lifetime parameters where the `TODO` comments
//! say, and implement the methods. Pay attention to the return type of
//! `next_token`: it must be `Option<&'a str>`, not `Option<&str>`, or the
//! last test will not compile.

/// Splits text into whitespace-separated tokens, one call at a time.
struct Tokenizer /* TODO: lifetime parameter */ {
    input: &str, // TODO: annotate
    pos: usize,
}

impl Tokenizer /* TODO */ {
    fn new(input: &str) -> Tokenizer {
        todo!()
    }

    /// The next token, or `None` when the input is exhausted. Skips any
    /// whitespace before the token.
    fn next_token(&mut self) -> Option<&str> {
        todo!()
    }

    /// Everything not yet consumed, including leading whitespace.
    fn rest(&self) -> &str {
        todo!()
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
