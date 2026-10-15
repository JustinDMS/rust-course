//! # A room of tiles
//!
//! Concept: enums as the vocabulary of a small world, `match` with data,
//! `let else` for early exits, and `Option` for lookups that can miss.
//!
//! Book: Chapter 6 (all of it)
//!
//! A room is a grid of tiles stored row by row. Doors carry the id of the
//! room they lead to; keys carry a colour.
//!
//! Your job: implement `Tile::from_char`, `Room::parse`, `Room::get`,
//! `Room::find_door` and `Room::is_walkable`.

#[derive(Debug, Clone, Copy, PartialEq)]
enum Colour {
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Door(u32),
    Key(Colour),
}

impl Tile {
    /// `'.'` floor, `'#'` wall, `'0'`..`'9'` a door with that id,
    /// `'r'` a red key, `'b'` a blue key. Anything else is `None`.
    /// Hint: `c.to_digit(10)` gives `Option<u32>`.
    fn from_char(c: char) -> Option<Tile> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Room {
    /// Parses a map like `"#.#\n#1#"`. Returns `None` if any character is
    /// not a tile, or if the rows differ in length. The empty string is an
    /// empty 0×0 room.
    fn parse(map: &str) -> Option<Room> {
        todo!()
    }

    /// The tile at `(x, y)`, or `None` when out of bounds.
    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        todo!()
    }

    /// The position of the door with the given id, if there is one.
    fn find_door(&self, id: u32) -> Option<(usize, usize)> {
        todo!()
    }

    /// Floors, doors and keys can be walked on; walls and out-of-bounds
    /// positions cannot. Use `let else` to handle the out-of-bounds case.
    fn is_walkable(&self, x: usize, y: usize) -> bool {
        todo!()
    }
}

const MAP: &str = "#####\n#..r#\n#.#1#\n##b##";

#[test]
fn chars_become_tiles() {
    assert_eq!(Tile::from_char('.'), Some(Tile::Floor));
    assert_eq!(Tile::from_char('#'), Some(Tile::Wall));
    assert_eq!(Tile::from_char('7'), Some(Tile::Door(7)));
    assert_eq!(Tile::from_char('r'), Some(Tile::Key(Colour::Red)));
    assert_eq!(Tile::from_char('b'), Some(Tile::Key(Colour::Blue)));
    assert_eq!(Tile::from_char('x'), None);
}

#[test]
fn parse_builds_a_grid() {
    let room = Room::parse(MAP).unwrap();
    assert_eq!(room.width, 5);
    assert_eq!(room.height, 4);
    assert_eq!(room.tiles.len(), 20);
    assert_eq!(room.tiles[0], Tile::Wall);
    assert_eq!(room.tiles[6], Tile::Floor);
    assert_eq!(Room::parse(""), Some(Room { width: 0, height: 0, tiles: vec![] }));
}

#[test]
fn parse_rejects_bad_maps() {
    assert_eq!(Room::parse("#x#"), None);
    assert_eq!(Room::parse("###\n##"), None);
}

#[test]
fn get_is_bounds_checked() {
    let room = Room::parse(MAP).unwrap();
    assert_eq!(room.get(3, 1), Some(Tile::Key(Colour::Red)));
    assert_eq!(room.get(3, 2), Some(Tile::Door(1)));
    assert_eq!(room.get(5, 0), None);
    assert_eq!(room.get(0, 4), None);
}

#[test]
fn doors_can_be_found() {
    let room = Room::parse(MAP).unwrap();
    assert_eq!(room.find_door(1), Some((3, 2)));
    assert_eq!(room.find_door(2), None);
}

#[test]
fn walkability() {
    let room = Room::parse(MAP).unwrap();
    assert!(room.is_walkable(1, 1));
    assert!(room.is_walkable(3, 1));
    assert!(room.is_walkable(3, 2));
    assert!(room.is_walkable(2, 3));
    assert!(!room.is_walkable(0, 0));
    assert!(!room.is_walkable(2, 2));
    assert!(!room.is_walkable(99, 99));
}
