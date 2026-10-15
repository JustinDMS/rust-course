//! # Entities in a game world
//!
//! Concept: structs that contain other structs, `Copy` for small
//! plain-data types, `&mut self` for state changes, and a free function
//! that scans a slice of structs.
//!
//! Book: Chapter 5 (all of it)
//!
//! Your job: implement every `todo!()`.

/// A position in the world. Small and plain, so it is `Copy`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Straight-line distance to `other`. Hint: `f32::sqrt`, or `.sqrt()`.
    fn distance_to(&self, other: Vec3) -> f32 {
        todo!()
    }
}

#[derive(Debug)]
struct Entity {
    name: String,
    position: Vec3,
    hp: u32,
    max_hp: u32,
}

impl Entity {
    /// A new entity at full health.
    fn new(name: &str, position: Vec3, max_hp: u32) -> Entity {
        todo!()
    }

    /// Reduces `hp` by `amount`, never below zero.
    /// Hint: integers have `saturating_sub`.
    fn take_damage(&mut self, amount: u32) {
        todo!()
    }

    /// Increases `hp` by `amount`, never above `max_hp`.
    fn heal(&mut self, amount: u32) {
        todo!()
    }

    fn is_alive(&self) -> bool {
        todo!()
    }

    /// Distance between this entity and another.
    fn distance_to(&self, other: &Entity) -> f32 {
        todo!()
    }
}

/// The index of the entity closest to `point`, or `None` if the slice is
/// empty. Dead entities do not count.
fn closest_alive(entities: &[Entity], point: Vec3) -> Option<usize> {
    todo!()
}

#[test]
fn distances() {
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(a.distance_to(b), 5.0);
    assert_eq!(b.distance_to(a), 5.0);
    assert_eq!(a.distance_to(a), 0.0);
}

#[test]
fn new_entities_are_at_full_health() {
    let e = Entity::new("Samus", Vec3::new(1.0, 2.0, 3.0), 99);
    assert_eq!(e.name, "Samus");
    assert_eq!(e.position, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(e.hp, 99);
    assert_eq!(e.max_hp, 99);
    assert!(e.is_alive());
}

#[test]
fn damage_and_healing_are_clamped() {
    let mut e = Entity::new("Metroid", Vec3::new(0.0, 0.0, 0.0), 50);
    e.take_damage(20);
    assert_eq!(e.hp, 30);
    e.heal(100);
    assert_eq!(e.hp, 50);
    e.take_damage(80);
    assert_eq!(e.hp, 0);
    assert!(!e.is_alive());
}

#[test]
fn entity_distance_uses_positions() {
    let a = Entity::new("a", Vec3::new(0.0, 0.0, 0.0), 1);
    let b = Entity::new("b", Vec3::new(0.0, 6.0, 8.0), 1);
    assert_eq!(a.distance_to(&b), 10.0);
}

#[test]
fn closest_alive_skips_the_dead() {
    let mut entities = vec![
        Entity::new("far", Vec3::new(10.0, 0.0, 0.0), 10),
        Entity::new("near", Vec3::new(1.0, 0.0, 0.0), 10),
        Entity::new("mid", Vec3::new(5.0, 0.0, 0.0), 10),
    ];
    let origin = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(closest_alive(&entities, origin), Some(1));
    entities[1].take_damage(10);
    assert_eq!(closest_alive(&entities, origin), Some(2));
    assert_eq!(closest_alive(&[], origin), None);
}
