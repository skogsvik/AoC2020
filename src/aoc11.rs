pub use crate::loaders::file_to_lines as load;
use itertools::Itertools;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

pub const DATA: &str = "input/aoc11";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 2441)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 2190)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load(DATA).collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

#[derive(Eq)]
struct Seat {
    pos: Coord, // Needs to be tuple so we can borrow without tons of boilerplate
    occupied: bool,
    neighbors: HashSet<Coord>,
}

impl Hash for Seat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Seat) -> bool {
        self.pos == other.pos
    }
}

impl Borrow<Coord> for Seat {
    fn borrow(&self) -> &Coord {
        &self.pos
    }
}

impl Seat {
    fn iter_neighbors<'a>(
        &'a self,
        seats: &'a HashMap<Coord, Self>,
    ) -> impl Iterator<Item = &'a Self> {
        self.neighbors
            .iter()
            .map(move |pos| seats.get(pos).unwrap())
    }
}

type Coord = (usize, usize);

fn parse_seats(
    input: impl Iterator<Item = String>,
    neighbor_finder: fn(seats: HashSet<Coord>) -> HashMap<Coord, HashSet<Coord>>,
) -> HashMap<Coord, Seat> {
    let seats = input
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|c| (row, c))
                .collect::<Vec<_>>() // Not sure how to avoid this allocation
        })
        .filter_map(|(row, (col, symbol))| {
            if symbol == 'L' {
                Some((row, col))
            } else {
                None
            }
        })
        .collect();

    neighbor_finder(seats)
        .drain()
        .map(|(pos, neighbors)| {
            (
                pos,
                Seat {
                    pos,
                    neighbors,
                    occupied: true,
                },
            )
        })
        .collect()
}

fn find_visible_neighors(seats: HashSet<Coord>) -> HashMap<Coord, HashSet<Coord>> {
    let rmax = *seats.iter().map(|(row, _)| row).max().unwrap() as isize;
    let cmax = *seats.iter().map(|(_, col)| col).max().unwrap() as isize;
    seats
        .iter()
        .map(|(row, col)| {
            let mut neigbors = HashSet::new();
            for (dx, dy) in (-1isize..=1)
                .cartesian_product(-1isize..=1)
                .filter(|pos| pos != &(0, 0))
            {
                for n in 1.. {
                    let r = *row as isize + n * dy;
                    let c = *col as isize + n * dx;
                    if r < 0 || c < 0 || r > rmax || c > cmax {
                        // None found
                        break;
                    }
                    if seats.contains(&(r as usize, c as usize)) {
                        neigbors.insert((r as usize, c as usize));
                        break;
                    }
                }
            }

            ((*row, *col), neigbors)
        })
        .collect()
}

fn find_adjacent_neighbors<'a>(seats: HashSet<Coord>) -> HashMap<Coord, HashSet<Coord>> {
    seats
        .iter()
        .map(|pos| {
            let neighbors: HashSet<_> = (0..3)
                .cartesian_product(0..3)
                .filter(|pos| pos != &(1, 1)) // Grid neighbors, except center
                .filter_map(|(dr, dc)| {
                    // Ignore out of bounds, either by failing checked sub or not existing in map
                    let adj_row = (pos.0 + dr).checked_sub(1)?;
                    let adj_col = (pos.1 + dc).checked_sub(1)?;
                    if seats.contains(&(adj_row, adj_col)) {
                        Some((adj_row, adj_col))
                    } else {
                        None
                    }
                })
                .collect();
            (*pos, neighbors)
        })
        .collect()
}

fn game_of_lifeish(seats: &mut HashMap<Coord, Seat>, max_neigbors: usize) -> bool {
    let mut changed_seats: HashSet<_> = seats
        .iter()
        .filter_map(|(pos, seat)| {
            if seat.occupied {
                // print!("has seat");
                let n_occupied = seat
                    .iter_neighbors(seats)
                    .filter(|seat| seat.occupied)
                    .count();
                // print!(" ({}) ", &n_occupied);
                if n_occupied >= max_neigbors {
                    // println!(" -> empty");
                    return Some(pos);
                }
                // println!();
            } else {
                // print!("no seat");
                if !seat.iter_neighbors(seats).any(|seat| seat.occupied) {
                    // println!(" -> occupied");
                    return Some(pos);
                }
                // println!();
            };
            None
        })
        .cloned()
        .collect();

    if changed_seats.is_empty() {
        return false; // No change
    }

    // Flip change seats
    changed_seats
        .drain()
        .for_each(|pos| seats.get_mut(&pos).unwrap().occupied ^= true);
    true
}

pub fn answer1(seat_map: impl Iterator<Item = String>) -> usize {
    let mut seats = parse_seats(seat_map, find_adjacent_neighbors);
    while game_of_lifeish(&mut seats, 4) {
        // println!("{:?}", mask.values().filter(|v| **v).count())
    }
    seats.values().filter(|seat| seat.occupied).count()
}

pub fn answer2(seat_map: impl Iterator<Item = String>) -> usize {
    let mut seats = parse_seats(seat_map, find_visible_neighors);
    while game_of_lifeish(&mut seats, 5) {
        // println!("{:?}", mask.values().filter(|v| **v).count())
    }
    seats.values().filter(|seat| seat.occupied).count()
}
