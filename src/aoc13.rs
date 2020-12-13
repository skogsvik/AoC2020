use crate::loaders::file_to_lines;
use modinverse::modinverse;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        let (earliest, buses) = load("input/aoc13");
        assert_eq!(answer1(&earliest, &buses), 2045)
    }
    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load("input/aoc13").1), 402251700208309)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let (earliest, buses) = load("input/aoc13");
        b.iter(|| answer1(&earliest, &buses));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let (_, buses) = load("input/aoc13");
        b.iter(|| answer2(&buses));
    }
}

pub fn load(filename: impl AsRef<std::path::Path>) -> (u32, Vec<Option<u32>>) {
    let mut lines = file_to_lines(filename);
    let time = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u32>().ok())
        .collect();
    (time, buses)
}

fn chinese_remainder(a: &[i64], mods: &[u64]) -> u64 {
    let total_mod: u64 = mods.iter().product();
    a.iter()
        .zip(mods.iter())
        .map(|(a_i, mod_i)| {
            let all_mod_but_i = (total_mod / mod_i) as i64;
            let inv_all_mod_but_i = modinverse(all_mod_but_i, *mod_i as i64).unwrap();
            *a_i as i64 * all_mod_but_i * inv_all_mod_but_i
        })
        .sum::<i64>() as u64
        % total_mod
}

pub fn answer1(earliest: &u32, buses: &[Option<u32>]) -> u32 {
    let (bus, wait_time) = buses
        .iter()
        .filter_map(|&bus| {
            let bus = bus?;
            Some((bus, bus - (earliest % bus)))
        })
        .min_by_key(|(_, wait)| *wait)
        .unwrap();
    bus * wait_time
}

pub fn answer2(buses: &[Option<u32>]) -> u64 {
    let a: Vec<_> = buses
        .iter()
        .enumerate()
        .filter_map(|(i, &bus)| {
            let bus = bus? as i64;
            Some((bus - i as i64) % bus)
        })
        .collect();
    dbg!(&a);
    let m: Vec<_> = buses.iter().filter_map(|&bus| Some(bus? as u64)).collect();
    chinese_remainder(&a, &m)
}
