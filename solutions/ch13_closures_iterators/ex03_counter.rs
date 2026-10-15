//! # Implementing Iterator (solution)

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct Fib {
    current: u64,
    next: u64,
}

impl Fib {
    fn new() -> Fib {
        Fib { current: 0, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.current;
        self.current = self.next;
        self.next += value;
        Some(value)
    }
}

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
