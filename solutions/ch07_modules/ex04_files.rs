//! # Modules in separate files (solution)
//!
//! Two changes: `pub mod items` so the path `inventory::items::Item` is
//! public, and `pub` on both `Item` fields so that `total_weight` (in the
//! parent module) and the tests (outside the module tree) can read them.
//!
//! Book: 7.5 "Separating Modules into Different Files"

// ===== src/lib.rs =====
pub mod inventory {
    // ===== src/inventory.rs =====
    pub mod items {
        // ===== src/inventory/items.rs =====
        #[derive(Debug, Clone, PartialEq)]
        pub struct Item {
            pub name: String,
            pub weight: u32,
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
