//! # The guessing game, as a library (solution)

use std::cmp::Ordering;

/// Turns one line of input into a guess.
pub fn parse_guess(input: &str) -> Option<u32> {
    match input.trim().parse() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

/// How `guess` relates to `secret`.
pub fn compare(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

/// The message the player sees for a comparison result.
pub fn feedback(ordering: Ordering) -> &'static str {
    match ordering {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

/// Plays a whole game against a scripted list of input lines.
pub fn play(inputs: &[&str], secret: u32) -> Vec<String> {
    let mut messages = Vec::new();
    for line in inputs {
        let guess = match parse_guess(line) {
            Some(guess) => guess,
            None => {
                messages.push(String::from("Please type a number!"));
                continue;
            }
        };
        let ordering = compare(guess, secret);
        messages.push(feedback(ordering).to_string());
        if ordering == Ordering::Equal {
            break;
        }
    }
    messages
}
