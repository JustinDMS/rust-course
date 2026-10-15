//! # Implementing Iterator
//!
//! Concept: implement `next` and every adaptor comes for free.
//!
//! Book: 13.2 "Creating Our Own Iterators with the Iterator Trait"
//!
//! Your job: implement `Iterator` for `Counter` (1 to 5, then `None`) and
//! for `Fib` (the Fibonacci sequence, forever).

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// TODO: impl Iterator for Counter { type Item = u32; fn next ... }

/// 0, 1, 1, 2, 3, 5, ... without end.
struct Fib {
    current: u64,
    next: u64,
}

impl Fib {
    fn new() -> Fib {
        Fib { current: 0, next: 1 }
    }
}

// TODO: impl Iterator for Fib.

#[test]
fn counter_counts_to_five() {
    let mut c = Counter::new();
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
    assert_eq!(c.next(), None);
}

#[test]
fn counter_gets_the_adaptors_for_free() {
    // The book's example: pairs (1,2) (2,3) (3,4) (4,5), multiplied,
    // keep those divisible by 3, sum: 2*3 + 3*4 ... → 6 + 12 = 18.
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}

#[test]
fn fib_is_infinite_so_take() {
    let first_ten: Vec<u64> = Fib::new().take(10).collect();
    assert_eq!(first_ten, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    let first_even_over_100 = Fib::new().find(|f| f % 2 == 0 && *f > 100);
    assert_eq!(first_even_over_100, Some(144));
}
