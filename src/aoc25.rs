pub use crate::loaders::file_to as load;
use std::iter::successors;

pub const DATA: &str = "input/aoc25";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 16457981)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let decks: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(decks.iter().cloned()));
    }

}

fn loop_key(subject: &u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1, |val, _| (val * subject) % 20201227)
}

fn find_pub_key_loop_size(subject: u64, target: &u64) -> u64 {
    successors(Some(1), |val| Some((val * subject) % 20201227))
        .take_while(|val| val != target)
        .count() as u64
}

pub fn answer1(keys: impl Iterator<Item = u64>) -> u64 {
    let keys: Vec<_> = keys.collect();
    loop_key(&keys[1], find_pub_key_loop_size(7, &keys[0]))
}
