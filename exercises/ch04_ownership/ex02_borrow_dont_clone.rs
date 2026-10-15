//! # Borrow, don't clone
//!
//! Concept: a function that only needs to *look at* data should borrow it
//! (`&T`, or `&[T]` for a `Vec<T>`), not take ownership. Then the caller can
//! keep using the value, and nobody has to copy anything.
//!
//! Book: 4.2 "References and Borrowing"
//!
//! Your job: `report` receives a slice but the helpers demand owned vectors.
//! Make it compile and pass **without adding `.clone()` or `.to_vec()`
//! anywhere**: change the signatures of `total`, `average` and `highest`.
//! Do not change the tests.

fn total(scores: Vec<u32>) -> u32 {
    scores.iter().sum()
}

fn average(scores: Vec<u32>) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }
    total(scores) as f64 / scores.len() as f64
}

fn highest(scores: Vec<u32>) -> u32 {
    scores.iter().copied().max().unwrap_or(0)
}

/// Builds `"total=<t> avg=<a> best=<b>"` for a list of scores.
fn report(scores: &[u32]) -> String {
    let t = total(scores);
    let a = average(scores);
    let b = highest(scores);
    format!("total={t} avg={a:.1} best={b}")
}

#[test]
fn report_of_three_scores() {
    assert_eq!(report(&[10, 20, 60]), "total=90 avg=30.0 best=60");
}

#[test]
fn report_of_nothing() {
    assert_eq!(report(&[]), "total=0 avg=0.0 best=0");
}

#[test]
fn caller_keeps_its_vector() {
    let scores = vec![1, 2, 3];
    let text = report(&scores);
    assert_eq!(text, "total=6 avg=2.0 best=3");
    assert_eq!(scores, vec![1, 2, 3]);
}
