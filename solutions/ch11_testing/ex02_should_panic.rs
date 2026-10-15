//! # Panics the tests expect (solution)

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[test]
fn in_range_values_are_kept() {
    assert_eq!(Guess::new(1).value(), 1);
    assert_eq!(Guess::new(100).value(), 100);
    assert_eq!(Guess::new(42).value(), 42);
}

#[test]
#[should_panic(expected = "Guess value must be greater than or equal to 1, got 0.")]
fn zero_is_too_small() {
    Guess::new(0);
}

#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100, got 200.")]
fn two_hundred_is_too_big() {
    Guess::new(200);
}

#[test]
#[should_panic(expected = "greater than or equal to 1")]
fn negative_numbers_are_too_small() {
    Guess::new(-5);
}
