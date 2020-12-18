use lazy_static::lazy_static;
use regex::Regex;
pub use std::fs::read_to_string as load;
use std::{collections::HashMap, iter};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load("input/aoc14").unwrap()), 14954914379452)
    }
    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load("input/aoc14").unwrap()), 3415488160714)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input = load("input/aoc14").unwrap();
        b.iter(|| answer1(&input));
    }
    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input = load("input/aoc14").unwrap();
        b.iter(|| answer2(&input));
    }
}

fn decode1(code: &str) -> impl Iterator<Item = u64> + '_ {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"mask = (?P<mask>[01X]+)|mem\[(?P<mem>\d+)\] = (?P<val>\d+)").unwrap();
    }
    let mut mask = 0;
    let mut overwrite = 0;
    let mut mems_and_masks = HashMap::new();
    for mat in RE.captures_iter(code) {
        if let Some(m) = mat.name("mask") {
            let m = m.as_str();
            mask = u64::from_str_radix(&m.replace('0', "1").replace('X', "0"), 2).unwrap();
            overwrite = u64::from_str_radix(&m.replace('X', "0"), 2).unwrap();
        } else if let Some(mem) = mat.name("mem") {
            let value = &mat["val"].parse().unwrap();
            mems_and_masks.insert(mem.as_str(), value & (!mask) | overwrite);
        }
    }
    mems_and_masks.into_values()
}

fn float_mem_iter(mask: &u64, floating_bits: &[u64]) -> Box<dyn Iterator<Item = u64>> {
    if floating_bits.len() == 1 {
        Box::new(iter::once(*mask).chain(iter::once(mask ^ floating_bits[0])))
    } else {
        Box::new(
            float_mem_iter(mask, &floating_bits[1..]).chain(float_mem_iter(
                &(floating_bits[0] ^ mask),
                &floating_bits[1..],
            )),
        )
    }
}

fn decode2(code: &str) -> impl Iterator<Item = u64> + '_ {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"mask = (?P<mask>[01X]+)|mem\[(?P<mem>\d+)\] = (?P<val>\d+)").unwrap();
    }
    let mut mask = 0;
    let mut floating_bits = Vec::new();
    let mut mems_and_masks = HashMap::new();
    for mat in RE.captures_iter(code) {
        if let Some(m) = mat.name("mask") {
            let m = m.as_str();
            floating_bits = m
                .match_indices('X')
                .map(|(i, _)| 2u64.pow(35 - i as u32))
                .collect();
            mask = u64::from_str_radix(&m.replace('X', "0"), 2).unwrap();
        } else if let Some(mem) = mat.name("mem") {
            let value = &mat["val"].parse().unwrap();
            let mem: u64 = mem.as_str().parse().unwrap();
            for float_mem in float_mem_iter(&(mask | mem), &floating_bits) {
                mems_and_masks.insert(float_mem, *value);
            }
        }
    }
    mems_and_masks.into_values()
}

pub fn answer1(code: &str) -> u64 {
    decode1(code).sum()
}
pub fn answer2(code: &str) -> u64 {
    decode2(code).sum()
}
