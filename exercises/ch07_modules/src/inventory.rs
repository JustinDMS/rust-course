//! The inventory module. Its `items` submodule lives in `inventory/items.rs`
//! but is not declared yet, so that file is currently not part of the
//! crate at all.

/// The combined weight of some items.
pub fn total_weight(items: &[items::Item]) -> u32 {
    let mut total = 0;
    for item in items {
        total += item.weight;
    }
    total
}
