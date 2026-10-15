//! # The tests module (solution)
//!
//! `use super::*;` brings `internal_adder` and `helpers` into the tests
//! module. `helpers::double` had to become `pub` to be reachable from a
//! sibling module (privacy is about the module tree, and `tests` is not
//! inside `helpers`). A test function cannot take arguments; the value
//! becomes a local.

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

mod helpers {
    pub fn double(x: i32) -> i32 {
        x * 2
    }

    pub fn triple(x: i32) -> i32 {
        x * 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn adds_with_an_argument() {
        let extra = 5;
        assert_eq!(internal_adder(1, extra), 1 + extra);
    }
}
