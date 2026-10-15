//! # Exhaustive matching (solution)

#[derive(Debug, Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Water,
    Lava,
}

/// Enemy health per difficulty.
fn enemy_hp(difficulty: Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => 50,
        Difficulty::Normal => 100,
        Difficulty::Hard => 200,
    }
}

/// Damage taken per second while standing on a tile.
fn damage_per_second(tile: Tile) -> u32 {
    match tile {
        Tile::Floor | Tile::Wall | Tile::Water => 0,
        Tile::Lava => 25,
    }
}

/// Whether the player can move onto a tile.
fn walkable(tile: Tile) -> bool {
    match tile {
        Tile::Floor | Tile::Water | Tile::Lava => true,
        Tile::Wall => false,
    }
}

#[test]
fn hp_by_difficulty() {
    assert_eq!(enemy_hp(Difficulty::Easy), 50);
    assert_eq!(enemy_hp(Difficulty::Normal), 100);
    assert_eq!(enemy_hp(Difficulty::Hard), 200);
}

#[test]
fn lava_hurts() {
    assert_eq!(damage_per_second(Tile::Floor), 0);
    assert_eq!(damage_per_second(Tile::Wall), 0);
    assert_eq!(damage_per_second(Tile::Water), 0);
    assert_eq!(damage_per_second(Tile::Lava), 25);
}

#[test]
fn walls_block() {
    assert!(walkable(Tile::Floor));
    assert!(walkable(Tile::Water));
    assert!(walkable(Tile::Lava));
    assert!(!walkable(Tile::Wall));
}
