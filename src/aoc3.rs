pub use crate::loaders::parse_vecvec_of_trees as load;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load("input/aoc3")), 171)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load("input/aoc3")), 1206576000)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input = load("input/aoc3");
        b.iter(|| answer1(&input));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input = load("input/aoc3");
        b.iter(|| answer2(&input));
    }
}

fn count_trees(trees: &[Vec<bool>], right_step: usize, down_step: usize) -> usize {
    let width = trees[0].len();

    trees
        .iter()
        .step_by(down_step)
        .scan(0usize, move |i_col, row| {
            let current = row[*i_col];
            *i_col = (*i_col + right_step) % width;
            Some(current)
        })
        .filter(|is_tree| *is_tree)
        .count()
}

pub fn answer1(trees: &[Vec<bool>]) -> usize {
    count_trees(trees, 3, 1)
}

pub fn answer2(trees: &[Vec<bool>]) -> usize {
    let steps = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    steps.iter().map(|step| count_trees(trees, step[0], step[1])).product()
}
