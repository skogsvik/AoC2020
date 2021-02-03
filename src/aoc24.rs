pub use crate::loaders::file_to_lines as load;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub const DATA: &str = "input/aoc24";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 488)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 4118)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let tiles: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(tiles.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let tiles: Vec<_> = load(DATA).collect();
        b.iter(|| answer2(tiles.iter().cloned()));
    }
}

type Coord = (i32, i32);
type Tile = u8; // TODO: Enum?
type TileMap = HashMap<Coord, Tile>;
type TileSet = HashSet<Coord>;

fn steps_to_coordinate(line: &str) -> Coord {
    line.chars()
        .batching(|it| match it.next() {
            None => None,
            Some('e') => Some((1, 0)),
            Some('w') => Some((-1, 0)),
            Some('n') => match it.next() {
                Some('e') => Some((0, 1)),
                Some('w') => Some((-1, 1)),
                _ => panic!("Bad line"),
            },
            Some('s') => match it.next() {
                Some('e') => Some((1, -1)),
                Some('w') => Some((0, -1)),
                _ => panic!("Bad line"),
            },
            _ => panic!("Bad char"),
        })
        .fold1(|(x0, y0), (x1, y1)| (x0 + x1, y0 + y1))
        .unwrap()
}

fn iter_possible_neighbors(current_tile: &Coord) -> impl Iterator<Item = Coord> + '_ {
    const POSSIBLE_NEIGHBORS: [Coord; 6] = [(-1, 1), (0, 1), (-1, 0), (1, 0), (0, -1), (1, -1)];
    POSSIBLE_NEIGHBORS
        .iter()
        .map(move |(x, y)| (current_tile.0 + x, current_tile.1 + y))
}

fn iter_neighbors<'a>(
    current_tile: &'a Coord,
    flipped_tiles: &'a TileSet,
) -> impl Iterator<Item = Coord> + 'a {
    iter_possible_neighbors(current_tile).filter(move |coord| flipped_tiles.contains(&coord))
}

pub fn answer1(tiles: impl Iterator<Item = String>) -> u32 {
    let mut flipped_tiles = TileMap::new();
    for tile in tiles {
        *flipped_tiles.entry(steps_to_coordinate(&tile)).or_default() ^= 1;
    }
    flipped_tiles.into_values().map(u32::from).sum()
}
pub fn answer2(tiles: impl Iterator<Item = String>) -> usize {
    let mut flipped_tiles = TileMap::new();
    for tile in tiles {
        *flipped_tiles.entry(steps_to_coordinate(&tile)).or_default() ^= 1;
    }

    let mut flipped_tiles: TileSet = flipped_tiles
        .into_iter()
        .filter_map(|(coord, flipped)| match flipped {
            1 => Some(coord),
            0 => None,
            _ => panic!("What?"),
        })
        .collect();
    for _ in 0..100 {
        let mut to_flip: HashSet<_> = flipped_tiles
            .iter()
            .flat_map(|coord| iter_possible_neighbors(coord)) // black tiles and all their neighbors
            .chain(flipped_tiles.iter().cloned())
            .collect();

        to_flip.retain(|coord| {
            let n_neighbors = iter_neighbors(&coord, &flipped_tiles).count();
            if flipped_tiles.contains(&coord) {
                // Black
                // Flips if not 1 or 2 black neighbors
                !matches!(n_neighbors, 1..=2)
            } else {
                // White
                // Flips if exactly 2 black neighbors
                n_neighbors == 2
            }
        });

        for coord in to_flip {
            // Remove if in set, otherwise add
            if !flipped_tiles.remove(&coord) {
                flipped_tiles.insert(coord);
            }
        }
    }
    flipped_tiles.len()
}
