pub use crate::loaders::file_chars_as_digits as load;
use itertools::Itertools;
use std::iter;

pub const DATA: &str = "input/aoc23";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 82934675)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 474600314018)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let cups: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(cups.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let cups: Vec<_> = load(DATA).collect();
        b.iter(|| answer2(cups.iter().cloned()));
    }
}

fn cup_game_optim(cups: &mut Vec<usize>, current: usize, max: usize) {
    let first = cups[current];
    let second = cups[first];
    let third = cups[second];
    let mut target = current.checked_sub(1).unwrap_or(max);
    while target == first || target == second || target == third {
        target = target.checked_sub(1).unwrap_or(max);
    }
    cups[current] = cups[third];
    cups[third] = cups[target];
    cups[target] = first;
}

pub fn answer1(cups: impl Iterator<Item = u32>) -> usize {
    let mut cups = cups.map(|v| v as usize - 1).peekable(); // Let's ignore <32 bit systems
    let mut current = *cups.peek().unwrap();
    let mut cups: Vec<_> = cups
        .chain(iter::once(current))
        .tuple_windows::<(_, _)>()
        .sorted_by_key(|items| items.0)
        .map(|items| items.1)
        .collect();
    let max = cups.len() - 1;
    for _ in 0..100 {
        cup_game_optim(&mut cups, current, max);
        current = cups[current];
    }
    iter::successors(Some(cups[0]), move |v| match cups[*v] {
        0 => None,
        v => Some(v),
    })
    .map(|v| v + 1)
    .fold_first(|tot, v| tot * 10 + v)
    .unwrap()
}

pub fn answer2(cups: impl Iterator<Item = u32>) -> u64 {
    let mut cups = cups.map(|v| v as usize - 1).peekable(); // Let's ignore <32 bit systems
    let mut current = *cups.peek().unwrap();
    let mut cups: Vec<_> = cups
        .chain(9..1_000_000)
        .chain(iter::once(current))
        .tuple_windows::<(_, _)>()
        .sorted_by_key(|items| items.0)
        .map(|items| items.1)
        .collect();
    let max = cups.len() - 1;
    for _ in 0..10_000_000 {
        cup_game_optim(&mut cups, current, max);
        current = cups[current];
    }
    let first = cups[0];
    let second = cups[first];
    (first as u64 + 1) * (second as u64 + 1)
}
