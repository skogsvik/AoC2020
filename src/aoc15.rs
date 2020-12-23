pub use crate::loaders::delimited_file_to as load;
use std::collections::HashMap;

pub const DATA: &str = "input/aoc15";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load(DATA, b',').collect::<Vec<_>>()), 755)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load(DATA, b',').collect::<Vec<_>>()), 11962)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let code: Vec<_> = load(DATA, b',').collect();
        b.iter(|| answer1(&code));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let code: Vec<_> = load(DATA, b',').collect();
        b.iter(|| answer2(&code));
    }
}

struct MemGame {
    last_spoken: HashMap<u32, u32>,
    current_num: u32,
    current_idx: u32,
}

impl Iterator for MemGame {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let next_num = self
            .last_spoken
            .get(&self.current_num)
            .map(|prev_idx| self.current_idx - prev_idx)
            .unwrap_or(0);
        self.last_spoken.insert(self.current_num, self.current_idx);
        self.current_idx += 1;
        self.current_num = next_num;
        Some(next_num)
    }
}

fn iter_memgame(iterations: usize, code: &[u32]) -> u32 {
    let mut mem = MemGame {
        last_spoken: code
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, n)| (n, i as u32))
            .collect(),
        current_num: *code.last().unwrap(),
        current_idx: code.len() as u32 - 1,
    };
    mem.nth(iterations - (code.len() + 1)).unwrap()
}

pub fn answer1(code: &[u32]) -> u32 {
    iter_memgame(2020, code)
}

pub fn answer2(code: &[u32]) -> u32 {
    iter_memgame(30000000, code) // There must be a trick I am missing here. Luckily Rust is fast!
}
