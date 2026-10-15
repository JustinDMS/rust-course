//! # Loops to iterators
//!
//! Concept: the same computations as iterator chains. The tests already
//! pass; the point is the rewrite. The checker rejects `for` and `while`
//! in this file, and `.len()`-based indexing is discouraged.
//!
//! Book: 13.3 "Comparing Performance: Loops vs. Iterators"
//!
//! Your job: rewrite each body as an iterator chain with the same
//! behaviour. Useful tools: `filter`, `count`, `map`, `max`, `scan`,
//! `windows`, `all`, `any`, `collect`.

/// How many words equal `needle`.
fn count_matching(words: &[&str], needle: &str) -> usize {
    let mut n = 0;
    for w in words {
        if *w == needle {
            n += 1;
        }
    }
    n
}

/// The largest absolute value, or `None` if empty.
fn max_abs(values: &[i32]) -> Option<i32> {
    let mut best = None;
    for &v in values {
        let a = v.abs();
        if best.is_none() || Some(a) > best {
            best = Some(a);
        }
    }
    best
}

/// Running totals: `[1, 2, 3]` → `[1, 3, 6]`.
fn running_total(values: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    let mut sum = 0;
    for &v in values {
        sum += v;
        out.push(sum);
    }
    out
}

/// Neighbouring pairs: `[1, 2, 3]` → `[(1, 2), (2, 3)]`.
fn pairs(values: &[i32]) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < values.len() {
        out.push((values[i], values[i + 1]));
        i += 1;
    }
    out
}

/// True if every value is positive (vacuously true for no values).
fn all_positive(values: &[i32]) -> bool {
    for &v in values {
        if v <= 0 {
            return false;
        }
    }
    true
}

#[test]
fn counting() {
    assert_eq!(count_matching(&["a", "b", "a"], "a"), 2);
    assert_eq!(count_matching(&[], "a"), 0);
}

#[test]
fn absolute_maximum() {
    assert_eq!(max_abs(&[3, -7, 5]), Some(7));
    assert_eq!(max_abs(&[]), None);
}

#[test]
fn running_totals() {
    assert_eq!(running_total(&[1, 2, 3]), vec![1, 3, 6]);
    assert!(running_total(&[]).is_empty());
}

#[test]
fn neighbours() {
    assert_eq!(pairs(&[1, 2, 3]), vec![(1, 2), (2, 3)]);
    assert!(pairs(&[1]).is_empty());
}

#[test]
fn positivity() {
    assert!(all_positive(&[1, 2]));
    assert!(!all_positive(&[1, 0]));
    assert!(all_positive(&[]));
}
