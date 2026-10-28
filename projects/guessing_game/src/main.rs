//! The playable guessing game. Run it with `cargo run -p guessing_game`.
//!
//! This file is complete; the functions it calls live in `lib.rs` and are
//! yours to implement during chapter 2.

use guessing_game::{compare, feedback, parse_guess};
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let Some(guess) = parse_guess(&guess) else {
            println!("Please type a number!");
            continue;
        };

        println!("You guessed: {guess}");

        let ordering = compare(guess, secret_number);
        println!("{}", feedback(ordering));
        if ordering == Ordering::Equal {
            break;
        }
    }
}
