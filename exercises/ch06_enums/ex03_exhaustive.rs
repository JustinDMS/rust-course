//! # Exhaustive matching
//!
//! Concept: `match` must cover every case. The compiler tells you what is
//! missing; your job is to decide what those cases *mean*, not to silence
//! the error with `_ => todo!()`.
//!
//! Book: 6.2 "The match Control Flow Construct"
//!
//! A new `Hard` difficulty was added to the game and two `Lava` tiles to
//! the map. Nothing compiles any more.
//!
//! Your job: make the tests pass. Do not use a `_` catch-all in
//! `enemy_hp`; spell out every variant.

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

/// Enemy health per difficulty: 50 on easy, 100 on normal, 200 on hard.
fn enemy_hp(difficulty: Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => 50,
        Difficulty::Normal => 100,
    }
}

/// Damage taken per second while standing on a tile: 0 for floor and
/// water, 25 for lava. Walls cannot be stood on, so also 0.
fn damage_per_second(tile: Tile) -> u32 {
    match tile {
        Tile::Floor | Tile::Wall => 0,
        Tile::Water => 0,
    }
}

/// Whether the player can move onto a tile. Lava is dangerous but walkable.
fn walkable(tile: Tile) -> bool {
    match tile {
        Tile::Floor => true,
        Tile::Water => true,
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
