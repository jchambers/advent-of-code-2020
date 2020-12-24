use std::ops::{Add, AddAssign};
use crate::tile::HexDirection::*;
use std::iter::Sum;
use std::collections::HashSet;

pub fn count_flipped_tiles(direction_lines: Vec<String>) -> usize {
    let mut flipped_tiles = HashSet::new();

    direction_lines.iter()
        .map(|line| HexDirection::parse_line(line.as_str()))
        .map(|directions| HexVector::from(directions))
        .for_each(|hex_vector| {
            if flipped_tiles.contains(&hex_vector) {
                flipped_tiles.remove(&hex_vector);
            } else {
                flipped_tiles.insert(hex_vector);
            }
        });

    flipped_tiles.len()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct HexVector {
    q: i32,
    r: i32,
}

#[derive(Debug, Eq, PartialEq)]
enum HexDirection {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Default for HexVector {
    fn default() -> Self {
        HexVector {
            q: 0,
            r: 0,
        }
    }
}

impl From<&HexDirection> for HexVector {
    fn from(direction: &HexDirection) -> Self {
        let (q, r) = match direction {
            HexDirection::East =>      ( 1,  0),
            HexDirection::SouthEast => ( 0,  1),
            HexDirection::SouthWest => (-1,  1),
            HexDirection::West =>      (-1,  0),
            HexDirection::NorthWest => ( 0, -1),
            HexDirection::NorthEast => ( 1, -1),
        };

        HexVector {
            q,
            r,
        }
    }
}

impl From<Vec<HexDirection>> for HexVector {
    fn from(directions: Vec<HexDirection>) -> Self {
        directions.iter()
            .map(|direction| HexVector::from(direction))
            .sum()
    }
}

impl Add for HexVector {
    type Output = HexVector;

    fn add(self, other: Self) -> Self::Output {
        HexVector {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl AddAssign for HexVector {

    fn add_assign(&mut self, other: Self) {
        self.q += other.q;
        self.r += other.r;
    }
}

impl Sum for HexVector {

    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut sum = HexVector::default();

        iter.for_each(|vector| sum += vector);

        sum
    }
}

impl HexDirection {

    fn parse_line(line: &str) -> Vec<HexDirection> {
        let mut offset = 0;
        let mut directions = Vec::new();

        while offset < line.len() {
            let (direction, delta_offset) = match &line[offset..(offset + 1)] {
                "e" => (East, 1),
                "w" => (West, 1),
                _ => {
                    match &line[offset..(offset + 2)] {
                        "se" => (SouthEast, 2),
                        "sw" => (SouthWest, 2),
                        "nw" => (NorthWest, 2),
                        "ne" => (NorthEast, 2),
                        _ => unreachable!()
                    }
                }
            };

            directions.push(direction);
            offset += delta_offset
        }

        directions
    }
}

#[cfg(test)]
mod test {
    use crate::tile::{HexVector, HexDirection};
    use crate::tile::HexDirection::{East, SouthEast, NorthEast, NorthWest, West, SouthWest};
    use crate::tile;

    #[test]
    fn add_hex_vector() {
        assert_eq!(HexVector { q: 1, r: 1 }, HexVector { q: 1, r: 0 } + HexVector { q: 0, r: 1});
    }

    #[test]
    fn parse_line() {
        assert_eq!(vec![East, SouthEast, NorthEast, East], HexDirection::parse_line("esenee"));
        assert_eq!(vec![NorthWest, West, SouthWest, East, East], HexDirection::parse_line("nwwswee"));
    }

    #[test]
    fn from_direction_list() {
        assert_eq!(HexVector { q: 0, r: 1 }, HexVector::from(HexDirection::parse_line("esew")));
        assert_eq!(HexVector::default(), HexVector::from(HexDirection::parse_line("nwwswee")));
    }

    #[test]
    fn count_flipped_tiles() {
        let direction_lines = vec![
            String::from("sesenwnenenewseeswwswswwnenewsewsw"),
            String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
            String::from("seswneswswsenwwnwse"),
            String::from("nwnwneseeswswnenewneswwnewseswneseene"),
            String::from("swweswneswnenwsewnwneneseenw"),
            String::from("eesenwseswswnenwswnwnwsewwnwsene"),
            String::from("sewnenenenesenwsewnenwwwse"),
            String::from("wenwwweseeeweswwwnwwe"),
            String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
            String::from("neeswseenwwswnwswswnw"),
            String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
            String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
            String::from("sweneswneswneneenwnewenewwneswswnese"),
            String::from("swwesenesewenwneswnwwneseswwne"),
            String::from("enesenwswwswneneswsenwnewswseenwsese"),
            String::from("wnwnesenesenenwwnenwsewesewsesesew"),
            String::from("nenewswnwewswnenesenwnesewesw"),
            String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
            String::from("neswnwewnwnwseenwseesewsenwsweewe"),
            String::from("wseweeenwnesenwwwswnew"),
        ];

        assert_eq!(10, tile::count_flipped_tiles(direction_lines));
    }
}
