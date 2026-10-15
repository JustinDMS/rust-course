//! # Describing game objects
//!
//! Concept: a trait with required and default methods, implemented for
//! several types, used through generic functions with one or two bounds.
//!
//! Book: Chapter 10 (all of it)
//!
//! Your job: implement the two traits for both types, then the two
//! generic functions.

trait Describe {
    fn name(&self) -> String;

    /// A category word such as `"enemy"` or `"pickup"`.
    fn kind(&self) -> &'static str;

    /// `"<name> (<kind>)"`
    fn describe(&self) -> String {
        format!("{} ({})", self.name(), self.kind())
    }
}

trait Combatant {
    fn hp(&self) -> u32;
}

struct Enemy {
    species: String,
    hp: u32,
}

struct Pickup {
    item: String,
    amount: u32,
}

// TODO: impl Describe for Enemy: name is the species, kind is "enemy".
// TODO: impl Combatant for Enemy.
// TODO: impl Describe for Pickup: name is "<amount>x <item>", kind is
// "pickup". Pickups are not combatants.

/// `describe()` of every item.
fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    todo!()
}

/// The description of the item with the most hp, or `None` if empty.
/// Use a `where` clause for the two bounds.
fn toughest<T>(items: &[T]) -> Option<String> {
    todo!()
}

#[test]
fn enemies_describe_themselves() {
    let e = Enemy { species: String::from("Metroid"), hp: 40 };
    assert_eq!(e.describe(), "Metroid (enemy)");
    assert_eq!(e.hp(), 40);
}

#[test]
fn pickups_describe_themselves() {
    let p = Pickup { item: String::from("missile"), amount: 5 };
    assert_eq!(p.describe(), "5x missile (pickup)");
}

#[test]
fn describe_all_works_for_any_describable() {
    let pickups = vec![
        Pickup { item: String::from("missile"), amount: 5 },
        Pickup { item: String::from("energy"), amount: 1 },
    ];
    assert_eq!(describe_all(&pickups), vec!["5x missile (pickup)", "1x energy (pickup)"]);
    let enemies = vec![Enemy { species: String::from("Zoomer"), hp: 2 }];
    assert_eq!(describe_all(&enemies), vec!["Zoomer (enemy)"]);
}

#[test]
fn toughest_needs_both_traits() {
    let enemies = vec![
        Enemy { species: String::from("Zoomer"), hp: 2 },
        Enemy { species: String::from("Ridley"), hp: 900 },
        Enemy { species: String::from("Metroid"), hp: 40 },
    ];
    assert_eq!(toughest(&enemies), Some(String::from("Ridley (enemy)")));
    assert_eq!(toughest::<Enemy>(&[]), None);
}
