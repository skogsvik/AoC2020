pub use crate::loaders::file_to as load;
use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load("input/aoc1").collect()), Ok(121396))
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load("input/aoc1").collect()), Ok(73616634))
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input = load("input/aoc1").collect();
        b.iter(|| answer1(&input));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input = load("input/aoc1").collect();
        b.iter(|| answer2(&input));
    }
}

pub fn answer1(expenses: &HashSet<i32>) -> Result<i32, &'static str> {
    expenses
        .iter()
        .filter_map(|exp1| Some(expenses.get(&(2020 - exp1))? * exp1))
        .next()
        .ok_or("No expenses that sum to 2020")
}

pub fn answer2(expenses: &HashSet<i32>) -> Result<i32, &'static str> {
    expenses
        .iter()
        .tuple_combinations()
        .filter_map(|(exp1, exp2)| Some(expenses.get(&(2020 - exp1 - exp2))? * exp1 * exp2))
        .next()
        .ok_or("No expenses that sum to 2020")
}
