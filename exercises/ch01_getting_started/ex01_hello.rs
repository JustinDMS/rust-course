//! # Hello, compiler
//!
//! Concept: Rust's syntax is strict, and the compiler's error messages are
//! the best teacher you have. This file has three small mistakes.
//!
//! Book: 1.2 "Hello, World!"
//!
//! Your job: run the tests, read the first error, fix it, repeat. Do not
//! change the tests.

/// Builds the classic greeting.
fn hello() -> String {
    let greeting = String::from("Hello, world!")
    println("{greeting}");
    greeting
}

/// Says hello to a specific person.
fn hello_to(name: &str) -> String {
    let message = format!("Hello, {name}!");
    println!("{message}")
    message
}

#[test]
fn hello_is_the_classic_text() {
    assert_eq!(hello(), "Hello, world!");
}

#[test]
fn hello_to_uses_the_name() {
    assert_eq!(hello_to("Samus"), "Hello, Samus!");
    assert_eq!(hello_to(""), "Hello, !");
}
