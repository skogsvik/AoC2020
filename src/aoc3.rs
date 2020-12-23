pub use crate::loaders::file_to_lines as load;
use lazy_static::lazy_static;

pub const DATA: &str = "input/aoc3";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 171)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 1206576000)
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

fn tree_map_to_mask(tree_map: impl Iterator<Item = String>) -> Vec<Vec<bool>> {
    tree_map
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_trees(trees: &[Vec<bool>], right_step: usize, down_step: usize) -> usize {
    let width = trees[0].len();

    trees
        .iter()
        .step_by(down_step)
        .scan(0usize, move |i_col, row| {
            let current = row[*i_col];
            *i_col = (*i_col + right_step) % width; // Map is periodic in width
            Some(current)
        })
        .filter(|is_tree| *is_tree)
        .count()
}

pub fn answer1(tree_map: impl Iterator<Item = String>) -> usize {
    count_trees(&tree_map_to_mask(tree_map), 3, 1)
}

pub fn answer2(tree_map: impl Iterator<Item = String>) -> usize {
    lazy_static! {
        // All the searchable steps
        static ref STEPS: [[usize; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    }
    let tree_mask = tree_map_to_mask(tree_map);
    STEPS
        .iter()
        .map(|step| count_trees(&tree_mask, step[0], step[1]))
        .product()
}
