//! # Lifetime annotations (solution)

/// The longer of two strings (the first on a tie).
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// Always returns `x`; `y` is only logged.
fn pick_first<'a>(x: &'a str, y: &str) -> &'a str {
    let _ = y.len();
    x
}

/// The first sentence of a text, borrowed from it.
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(text: &'a str) -> Excerpt<'a> {
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
