//! # Paths: crate, super, self and use (solution)
//!
//! `stove::cook` is a sibling of `recipes`, so it reaches it through the
//! parent: `super::recipes::soup()`. The root-level `dinner` needs the full
//! `kitchen::recipes::bread()`. Two imports both named `Result` clash;
//! rename one with `as` (or import only the one you use).

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
            format!("cooking {}", super::recipes::soup())
        }
    }

    pub fn menu() -> Vec<&'static str> {
        vec![recipes::soup(), recipes::bread()]
    }
}

mod dining {
    use std::io::Result as IoResult;

    /// A write that may fail with an I/O error.
    pub fn write_receipt(buf: &mut Vec<u8>) -> IoResult<()> {
        use std::io::Write;
        buf.write_all(b"receipt")
    }
}

/// Dinner is cooked soup followed by bread.
fn dinner() -> String {
    format!("{}, then {}", kitchen::stove::cook(), kitchen::recipes::bread())
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
