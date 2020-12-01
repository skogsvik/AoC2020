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
    let bad_expense = expenses
        .iter()
        .find(|expense| expenses.contains(&(2020 - *expense)))
        .ok_or("No expenses that sum to 2020")?;
    Ok(bad_expense * (2020 - bad_expense))
}

pub fn answer2(expenses: &HashSet<i32>) -> Result<i32, &'static str> {
    let (bad_exp1, bad_exp2) = expenses
        .iter()
        .tuple_combinations()
        .find(|(exp1, exp2)| expenses.contains(&(2020 - *exp1 - *exp2)))
        .ok_or("No exp triplets that sum to 2020")?;

    Ok(bad_exp1 * bad_exp2 * (2020 - bad_exp1 - bad_exp2))
}
