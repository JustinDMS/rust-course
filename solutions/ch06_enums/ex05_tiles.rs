//! # A room of tiles (solution)

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
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            'r' => Some(Tile::Key(Colour::Red)),
            'b' => Some(Tile::Key(Colour::Blue)),
            '0'..='9' => Some(Tile::Door(c.to_digit(10)?)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Room {
    fn parse(map: &str) -> Option<Room> {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in map.lines() {
            let row_len = line.chars().count();
            if height == 0 {
                width = row_len;
            } else if row_len != width {
                return None;
            }
            for c in line.chars() {
                let Some(tile) = Tile::from_char(c) else {
                    return None;
                };
                tiles.push(tile);
            }
            height += 1;
        }
        Some(Room { width, height, tiles })
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.tiles[y * self.width + x])
    }

    fn find_door(&self, id: u32) -> Option<(usize, usize)> {
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Tile::Door(found) = tile {
                if *found == id {
                    return Some((i % self.width, i / self.width));
                }
            }
        }
        None
    }

    fn is_walkable(&self, x: usize, y: usize) -> bool {
        let Some(tile) = self.get(x, y) else {
            return false;
        };
        match tile {
            Tile::Floor | Tile::Door(_) | Tile::Key(_) => true,
            Tile::Wall => false,
        }
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
