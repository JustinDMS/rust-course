//! # Defining and implementing a trait (solution)

pub trait Summary {
    fn summarize_author(&self) -> String;

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

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

fn notify(item: &impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}

fn summarize_all<T: Summary>(items: &[T]) -> Vec<String> {
    let mut out = Vec::new();
    for item in items {
        out.push(item.summarize());
    }
    out
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
