//! # Building structs from other structs
//!
//! Concept: every field must be initialised; `..other` fills in the rest
//! from another *value* (not a reference) and moves what it takes;
//! `#[derive(Debug)]` is needed before `{:?}` works.
//!
//! Book: 5.1 "Defining and Instantiating Structs"
//!
//! Your job: three compile errors. Fix them without changing the tests.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// A fresh, active user who has signed in once.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
    }
}

/// A copy of `user` with a different email address.
fn with_email(user: &User, email: String) -> User {
    User { email, ..user }
}

/// An admin account that shares `user`'s email; `user` itself is returned
/// unchanged alongside it.
fn promote(user: User) -> (User, User) {
    let admin = User {
        username: String::from("admin"),
        email: user.email.clone(),
        ..user
    };
    (user, admin)
}

#[test]
fn build_user_fills_in_defaults() {
    let u = build_user(String::from("s@x.io"), String::from("samus"));
    assert!(u.active);
    assert_eq!(u.username, "samus");
    assert_eq!(u.email, "s@x.io");
    assert_eq!(u.sign_in_count, 1);
}

#[test]
fn with_email_keeps_everything_else() {
    let u = build_user(String::from("s@x.io"), String::from("samus"));
    let v = with_email(&u, String::from("aran@x.io"));
    assert_eq!(v.email, "aran@x.io");
    assert_eq!(v.username, "samus");
    assert_eq!(u.email, "s@x.io");
}

#[test]
fn promote_returns_both() {
    let u = build_user(String::from("s@x.io"), String::from("samus"));
    let (original, admin) = promote(u);
    assert_eq!(original.username, "samus");
    assert_eq!(admin.username, "admin");
    assert_eq!(admin.email, original.email);
    assert_eq!(admin.sign_in_count, 1);
}

#[test]
fn users_can_be_debug_printed() {
    let u = build_user(String::from("s@x.io"), String::from("samus"));
    let text = format!("{u:?}");
    assert!(text.starts_with("User {"));
    assert!(text.contains("username: \"samus\""));
}
