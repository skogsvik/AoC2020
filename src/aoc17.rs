pub use crate::loaders::file_to_lines as load;
use itertools::{repeat_n, Itertools};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc17")), 386)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc17")), 2276)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc17").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc17").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

type Coord = Vec<isize>;

fn parse_initial(input: impl Iterator<Item = String>, dims: usize) -> HashSet<Coord> {
    input
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| (vec![row as isize, col as isize], symbol))
                .collect::<Vec<_>>() // Not sure how to avoid this allocation
        })
        .filter_map(|(mut coord, symbol)| {
            if symbol == '#' {
                coord.resize(dims, 0);
                Some(coord)
            } else {
                None
            }
        })
        .collect()
}

fn iter_possible_neighbors(node: &[isize]) -> impl Iterator<Item = Coord> + '_ {
    repeat_n(-1isize..=1, node.len())
        .multi_cartesian_product()
        .filter(|coord| coord.iter().any(|i| i != &0))
        .map(move |delta| {
            delta
                .iter()
                .zip(node.iter())
                .map(|(di, i)| i + di)
                .collect()
        })
}

fn all_possible_neighbors_and_counts(nodes: &HashSet<Coord>) -> HashMap<Coord, usize> {
    let mut neighbors = HashMap::new();
    for node in nodes.iter().flat_map(|node| iter_possible_neighbors(node)) {
        *neighbors.entry(node).or_default() += 1;
    }
    neighbors
}

fn iter_alive_neighbors<'a>(
    nodes: &'a HashSet<Coord>,
    node: &'a [isize],
) -> impl Iterator<Item = Coord> + 'a {
    iter_possible_neighbors(node).filter(move |coord| nodes.contains(coord))
}

fn game_of_lifeish(nodes: &mut HashSet<Coord>) {
    let new_nodes: HashSet<_> = all_possible_neighbors_and_counts(nodes)
        .drain()
        .filter_map(|(nbor, count)| match count {
            3 => Some(nbor),
            _ => None,
        })
        .collect();

    let dead_nodes: HashSet<_> = nodes
        .iter()
        .filter(|pos| {
            let alive_neighbors = iter_alive_neighbors(nodes, pos).count();
            alive_neighbors != 2 && alive_neighbors != 3
        })
        .cloned()
        .collect();

    assert!(dead_nodes.iter().all(|dead_node| nodes.remove(dead_node)));
    nodes.extend(new_nodes);
}

pub fn answer1(node_map: impl Iterator<Item = String>) -> usize {
    let mut pocket_dim = parse_initial(node_map, 3);
    for _ in 0..6 {
        game_of_lifeish(&mut pocket_dim);
    }
    pocket_dim.len()
}

pub fn answer2(node_map: impl Iterator<Item = String>) -> usize {
    let mut pocket_dim = parse_initial(node_map, 4);
    for _ in 0..6 {
        game_of_lifeish(&mut pocket_dim);
    }
    pocket_dim.len()
}
