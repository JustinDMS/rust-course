//! # Paths: crate, super, self and use
//!
//! Concept: a path can be absolute (`crate::...`) or relative to the
//! current module; `super::` goes up one level; `use` shortcuts only apply
//! in the module where they are written; `as` renames a clashing import.
//!
//! Book: 7.3 and 7.4
//!
//! Your job: three compile errors about paths. Fix them without moving any
//! function to a different module and without changing the tests.

mod kitchen {
    pub mod recipes {
        pub fn soup() -> &'static str {
            "soup"
        }

        pub fn bread() -> &'static str {
            "bread"
        }
    }

    pub mod stove {
        /// Cooks today's soup.
        pub fn cook() -> String {
            format!("cooking {}", recipes::soup())
        }
    }

    pub fn menu() -> Vec<&'static str> {
        vec![recipes::soup(), recipes::bread()]
    }
}

mod dining {
    use std::fmt::Result;
    use std::io::Result;

    /// A write that may fail with an I/O error.
    pub fn write_receipt(buf: &mut Vec<u8>) -> Result<()> {
        use std::io::Write;
        buf.write_all(b"receipt")
    }
}

/// Dinner is cooked soup followed by bread.
fn dinner() -> String {
    format!("{}, then {}", kitchen::stove::cook(), recipes::bread())
}

#[test]
fn cooking() {
    assert_eq!(kitchen::stove::cook(), "cooking soup");
}

#[test]
fn menu_lists_both() {
    assert_eq!(kitchen::menu(), vec!["soup", "bread"]);
}

#[test]
fn receipts_write() {
    let mut buf = Vec::new();
    dining::write_receipt(&mut buf).unwrap();
    assert_eq!(buf, b"receipt");
}

#[test]
fn dinner_is_served() {
    assert_eq!(dinner(), "cooking soup, then bread");
}
