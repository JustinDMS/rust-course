//! # The guessing game, as a library
//!
//! The playable program is in `main.rs`; it reads lines from the terminal
//! and picks a random secret. Everything that can be *tested* lives here,
//! so the stages of chapter 2 can check your work piece by piece.
//!
//! Stage 1: `parse_guess`
//! Stage 2: `compare` and `feedback`
//! Stage 3: `play`
//!
//! Book: Chapter 2 "Programming a Guessing Game"

use std::cmp::Ordering;

/// Turns one line of input into a guess.
///
/// Trailing whitespace and the newline from `read_line` are ignored, so
/// `"42\n"` and `"  42 "` both give `Some(42)`. Anything that is not a
/// non-negative whole number gives `None`.
pub fn parse_guess(input: &str) -> Option<u32> {
    todo!()
}

/// How `guess` relates to `secret`: `Less`, `Greater` or `Equal`.
pub fn compare(guess: u32, secret: u32) -> Ordering {
    todo!()
}

/// The message the player sees for a comparison result.
///
/// `Less` → `"Too small!"`, `Greater` → `"Too big!"`, `Equal` → `"You win!"`.
pub fn feedback(ordering: Ordering) -> &'static str {
    todo!()
}

/// Plays a whole game against a scripted list of input lines and returns
/// every message the game would print, in order.
///
/// For each line: if it is not a number, the message is
/// `"Please type a number!"`; otherwise it is `feedback(compare(guess, secret))`.
/// The game stops after the first `"You win!"`; later lines are ignored.
pub fn play(inputs: &[&str], secret: u32) -> Vec<String> {
    todo!()
}
