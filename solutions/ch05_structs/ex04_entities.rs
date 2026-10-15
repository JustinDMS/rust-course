//! # Entities in a game world (solution)

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

    fn distance_to(&self, other: Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
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
    fn new(name: &str, position: Vec3, max_hp: u32) -> Entity {
        Entity {
            name: name.to_string(),
            position,
            hp: max_hp,
            max_hp,
        }
    }

    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }

    fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn distance_to(&self, other: &Entity) -> f32 {
        self.position.distance_to(other.position)
    }
}

/// The index of the living entity closest to `point`.
fn closest_alive(entities: &[Entity], point: Vec3) -> Option<usize> {
    let mut best: Option<usize> = None;
    let mut best_distance = f32::INFINITY;
    for (i, entity) in entities.iter().enumerate() {
        if !entity.is_alive() {
            continue;
        }
        let d = entity.position.distance_to(point);
        if d < best_distance {
            best_distance = d;
            best = Some(i);
        }
    }
    best
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
