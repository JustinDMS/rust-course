//! Stage 2: comparing and reporting.

use guessing_game::{compare, feedback};
use std::cmp::Ordering;

#[test]
fn compare_orders_guess_against_secret() {
    assert_eq!(compare(10, 50), Ordering::Less);
    assert_eq!(compare(90, 50), Ordering::Greater);
    assert_eq!(compare(50, 50), Ordering::Equal);
}

#[test]
fn feedback_matches_the_book() {
    assert_eq!(feedback(Ordering::Less), "Too small!");
    assert_eq!(feedback(Ordering::Greater), "Too big!");
    assert_eq!(feedback(Ordering::Equal), "You win!");
}
