//! # Median and mode (solution)

use std::collections::HashMap;

fn median(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort();
    let n = sorted.len();
    let median = if n % 2 == 1 {
        sorted[n / 2] as f64
    } else {
        (sorted[n / 2 - 1] as f64 + sorted[n / 2] as f64) / 2.0
    };
    Some(median)
}

fn mode(values: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    let mut best: Option<(i32, u32)> = None;
    for (&value, &count) in &counts {
        match best {
            None => best = Some((value, count)),
            Some((best_value, best_count)) => {
                if count > best_count || (count == best_count && value < best_value) {
                    best = Some((value, count));
                }
            }
        }
    }
    best.map(|(value, _)| value)
}

fn evens(values: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for &v in values {
        if v % 2 == 0 {
            out.push(v);
        }
    }
    out
}

#[test]
fn medians() {
    assert_eq!(median(&[3, 1, 2]), Some(2.0));
    assert_eq!(median(&[4, 1, 3, 2]), Some(2.5));
    assert_eq!(median(&[7]), Some(7.0));
    assert_eq!(median(&[]), None);
}

#[test]
fn modes() {
    assert_eq!(mode(&[1, 2, 2, 3]), Some(2));
    assert_eq!(mode(&[5, 5, 1, 1, 3]), Some(1));
    assert_eq!(mode(&[9]), Some(9));
    assert_eq!(mode(&[]), None);
}

#[test]
fn even_values() {
    assert_eq!(evens(&[1, 2, 3, 4, 6, 7]), vec![2, 4, 6]);
    assert!(evens(&[1, 3, 5]).is_empty());
}
