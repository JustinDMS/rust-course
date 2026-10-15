//! # Loops to iterators (solution)

fn count_matching(words: &[&str], needle: &str) -> usize {
    words.iter().filter(|&&w| w == needle).count()
}

fn max_abs(values: &[i32]) -> Option<i32> {
    values.iter().map(|v| v.abs()).max()
}

fn running_total(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .scan(0, |sum, &v| {
            *sum += v;
            Some(*sum)
        })
        .collect()
}

fn pairs(values: &[i32]) -> Vec<(i32, i32)> {
    values.windows(2).map(|w| (w[0], w[1])).collect()
}

fn all_positive(values: &[i32]) -> bool {
    values.iter().all(|&v| v > 0)
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
