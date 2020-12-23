pub use crate::loaders::file_to_lines as load;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::ops::{AddAssign, MulAssign};

pub const DATA: &str = "input/aoc18";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 18213007238947)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 388966573054664)
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

fn solve_new_math<T: Iterator<Item = char>>(statement: &mut T) -> u64 {
    let mut acc = 0;
    let mut op: fn(&mut u64, u64) = u64::add_assign;
    while let Some(c) = statement.next() {
        // TODO: write as iterator using itertools batches
        match c {
            '(' => op(&mut acc, solve_new_math(statement)),
            ')' => return acc,
            '+' => op = u64::add_assign,
            '*' => op = u64::mul_assign,
            c => op(&mut acc, c.to_digit(10).unwrap() as u64),
        }
    }
    acc
}

fn solve_sorted_math(mut statement: String) -> u64 {
    // There must be a scanning approach that I am too tired to figure out
    lazy_static! {
        // All the searchable steps
        static ref ADD: Regex = Regex::new(
            r"(\d+) ?\+ ?(\d+)"
        ).unwrap();
        static ref MUL_CHAIN: Regex = Regex::new(
            r"(?:^|\()([\d *]+)(?:$|\))"
        ).unwrap();

    }
    while statement.contains('+') || statement.contains('*') {
        let tmp = ADD.replace_all(&statement, |cap: &Captures| {
            (cap[1].parse::<u64>().unwrap() + cap[2].parse::<u64>().unwrap()).to_string()
        });
        statement = MUL_CHAIN
            .replace_all(&tmp, |cap: &Captures| {
                cap[1]
                    .split('*')
                    .map(|s| s.trim().parse::<u64>().unwrap())
                    .product::<u64>()
                    .to_string()
            })
            .into();
    }
    statement.parse().unwrap()
}

pub fn answer1(statements: impl Iterator<Item = String>) -> u64 {
    statements
        .map(|statement| solve_new_math(&mut statement.replace(' ', "").chars()))
        .sum()
}

pub fn answer2(statements: impl Iterator<Item = String>) -> u64 {
    statements.map(solve_sorted_math).sum()
}
