//! # Lifetime annotations
//!
//! Concept: when a function returns a reference and has more than one
//! reference parameter, you must say which input the result borrows from.
//! A struct holding a reference needs a lifetime parameter.
//!
//! Book: 10.3 "Validating References with Lifetimes"
//!
//! Your job: three compile errors, all `E0106: missing lifetime
//! specifier`. Add the annotations. Be precise: `pick_first` must only tie
//! its result to its first argument (one of the tests depends on that).

/// The longer of two strings (the first on a tie).
fn longest(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}

/// Always returns `x`; `y` is only logged.
fn pick_first(x: &str, y: &str) -> &str {
    let _ = y.len();
    x
}

/// The first sentence of a text, borrowed from it.
struct Excerpt {
    part: &str,
}

impl Excerpt {
    fn new(text: &str) -> Excerpt {
        let part = match text.find('.') {
            Some(i) => &text[..=i],
            None => text,
        };
        Excerpt { part }
    }

    fn part(&self) -> &str {
        self.part
    }
}

#[test]
fn longest_picks_the_longer() {
    assert_eq!(longest("abc", "de"), "abc");
    assert_eq!(longest("ab", "cde"), "cde");
    assert_eq!(longest("ab", "cd"), "ab");
}

#[test]
fn pick_first_result_outlives_the_second_argument() {
    let a = String::from("keep me");
    let result;
    {
        let b = String::from("temporary");
        result = pick_first(&a, &b);
    }
    assert_eq!(result, "keep me");
}

#[test]
fn excerpts_borrow_from_the_text() {
    let text = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt::new(&text);
    assert_eq!(excerpt.part(), "Call me Ishmael.");
    assert_eq!(Excerpt::new("no period").part(), "no period");
}
