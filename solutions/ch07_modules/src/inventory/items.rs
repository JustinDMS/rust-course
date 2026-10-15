//! Item definitions. Reached as `crate::inventory::items`.

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
