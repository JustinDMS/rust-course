//! # Borrow, don't clone (solution)
//!
//! Every helper only reads the scores, so each takes `&[u32]`, the same slice
//! `report` already has. A `&Vec<u32>` coerces to `&[u32]` at the call site,
//! and shared borrows can overlap freely.

fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn average(scores: &[u32]) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }
    total(scores) as f64 / scores.len() as f64
}

fn highest(scores: &[u32]) -> u32 {
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
