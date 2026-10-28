//! # Modules in separate files
//!
//! Concept: `mod name;` loads `name.rs` (or `name/mod.rs`); a submodule of
//! a file module lives in a directory named after it; and a module's items
//! are private to it unless marked `pub`, no matter which file they sit in.
//!
//! Book: 7.5 "Separating Modules into Different Files"
//!
//! This exercise is a small crate flattened into one file so that it fits in
//! one editor. Each `// ===== src/... =====` comment marks where a file
//! boundary would be. In a real project the `mod` lines would end in `;`
//! (`pub mod inventory;` in `lib.rs`, `pub mod items;` in `inventory.rs`)
//! and the bodies would live in their own files. Splitting into files
//! changes nothing about paths or privacy, which is the point.
//!
//! Your job: the `tests` module plays the role of an outside user of the
//! library and cannot reach what it needs. Make the `items` module public,
//! and the two `Item` fields it reads too. Do not change the tests.

// ===== src/lib.rs =====
// Really `pub mod inventory;`, with the body in src/inventory.rs:
pub mod inventory {
    // ===== src/inventory.rs =====
    // Really `pub mod items;`, with the body in src/inventory/items.rs.
    // Forget that line in a real crate and items.rs is not compiled at all.
    mod items {
        // ===== src/inventory/items.rs =====
        #[derive(Debug, Clone, PartialEq)]
        pub struct Item {
            name: String,
            weight: u32,
        }

        impl Item {
            pub fn new(name: &str, weight: u32) -> Item {
                Item {
                    name: name.to_string(),
                    weight,
                }
            }
        }
    }

    /// The combined weight of some items.
    pub fn total_weight(items: &[items::Item]) -> u32 {
        let mut total = 0;
        for item in items {
            total += item.weight;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    // An outside user may only follow `pub` paths.
    use crate::inventory::items::Item;
    use crate::inventory::total_weight;

    #[test]
    fn items_can_be_built_and_read() {
        let item = Item::new("missile", 2);
        assert_eq!(item.name, "missile");
        assert_eq!(item.weight, 2);
    }

    #[test]
    fn weights_add_up() {
        let items = vec![Item::new("missile", 2), Item::new("tank", 10)];
        assert_eq!(total_weight(&items), 12);
        assert_eq!(total_weight(&[]), 0);
    }
}
