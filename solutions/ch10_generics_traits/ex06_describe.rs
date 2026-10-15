//! # Describing game objects (solution)

trait Describe {
    fn name(&self) -> String;
    fn kind(&self) -> &'static str;

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

impl Describe for Enemy {
    fn name(&self) -> String {
        self.species.clone()
    }

    fn kind(&self) -> &'static str {
        "enemy"
    }
}

impl Combatant for Enemy {
    fn hp(&self) -> u32 {
        self.hp
    }
}

impl Describe for Pickup {
    fn name(&self) -> String {
        format!("{}x {}", self.amount, self.item)
    }

    fn kind(&self) -> &'static str {
        "pickup"
    }
}

fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    let mut out = Vec::new();
    for item in items {
        out.push(item.describe());
    }
    out
}

fn toughest<T>(items: &[T]) -> Option<String>
where
    T: Describe + Combatant,
{
    let mut best: Option<&T> = None;
    for item in items {
        match best {
            Some(b) if b.hp() >= item.hp() => {}
            _ => best = Some(item),
        }
    }
    best.map(|b| b.describe())
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
