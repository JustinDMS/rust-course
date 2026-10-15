//! # Hello, compiler (solution)
//!
//! Three fixes: a missing semicolon after the `let`, `println` without its
//! `!`, and a missing semicolon after the second `println!`.

/// Builds the classic greeting.
fn hello() -> String {
    let greeting = String::from("Hello, world!");
    println!("{greeting}");
    greeting
}

/// Says hello to a specific person.
fn hello_to(name: &str) -> String {
    let message = format!("Hello, {name}!");
    println!("{message}");
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
