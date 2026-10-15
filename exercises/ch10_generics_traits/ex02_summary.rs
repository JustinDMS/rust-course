//! # Defining and implementing a trait
//!
//! Concept: a trait with a required method and a default method, two
//! implementations (one overriding the default), and functions that take
//! `impl Trait`.
//!
//! Book: 10.2 "Traits: Defining Shared Behavior"
//!
//! Your job: implement the trait for both types and write `notify` and
//! `summarize_all`.

pub trait Summary {
    /// Who wrote it, e.g. `"@samus"` or `"Chozo Press"`.
    fn summarize_author(&self) -> String;

    /// A short description; by default `"(Read more from <author>...)"`.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

// TODO: impl Summary for Tweet. The author is "@<username>"; keep the
// default summarize.

// TODO: impl Summary for NewsArticle. The author is the author field;
// summarize is overridden to "<headline>, by <author>".

/// `"Breaking news! <summary>"`
fn notify(item: &impl Summary) -> String {
    todo!()
}

/// The summaries of every item in a slice of one summarizable type.
fn summarize_all<T: Summary>(items: &[T]) -> Vec<String> {
    todo!()
}

#[test]
fn tweets_use_the_default() {
    let t = Tweet { username: String::from("samus"), content: String::from("hi") };
    assert_eq!(t.summarize_author(), "@samus");
    assert_eq!(t.summarize(), "(Read more from @samus...)");
}

#[test]
fn articles_override() {
    let a = NewsArticle { headline: String::from("Metroids escape"), author: String::from("Chozo Press") };
    assert_eq!(a.summarize_author(), "Chozo Press");
    assert_eq!(a.summarize(), "Metroids escape, by Chozo Press");
}

#[test]
fn notify_takes_anything_summarizable() {
    let t = Tweet { username: String::from("x"), content: String::new() };
    let a = NewsArticle { headline: String::from("H"), author: String::from("A") };
    assert_eq!(notify(&t), "Breaking news! (Read more from @x...)");
    assert_eq!(notify(&a), "Breaking news! H, by A");
}

#[test]
fn summarize_all_maps_over_a_slice() {
    let tweets = vec![
        Tweet { username: String::from("a"), content: String::new() },
        Tweet { username: String::from("b"), content: String::new() },
    ];
    assert_eq!(
        summarize_all(&tweets),
        vec!["(Read more from @a...)", "(Read more from @b...)"]
    );
}
