//! Item definitions. Reached as `crate::inventory::items` once the parent
//! module declares it.

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
