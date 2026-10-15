//! # The tests module
//!
//! Concept: unit tests live in a `#[cfg(test)] mod tests` that imports its
//! parent with `use super::*;`. Test functions take no arguments. Private
//! items of the parent are reachable; private items of *other* modules
//! are not.
//!
//! Book: 11.3 "Test Organization"
//!
//! Your job: three compile errors in and around the tests module. Fix
//! them without changing what the tests check.

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

mod helpers {
    fn double(x: i32) -> i32 {
        x * 2
    }

    pub fn triple(x: i32) -> i32 {
        x * 3
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds_internally() {
        assert_eq!(internal_adder(2, 2), 4);
    }

    #[test]
    fn doubles_and_triples() {
        assert_eq!(helpers::double(2), 4);
        assert_eq!(helpers::triple(2), 6);
    }

    #[test]
    fn adds_with_an_argument(extra: i32) {
        assert_eq!(internal_adder(1, extra), 1 + extra);
    }
}
