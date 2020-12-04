use lazy_static::lazy_static;
use regex::{Captures, Regex};
pub use std::{fs::read_to_string as load, ops::RangeInclusive};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load("input/aoc4").unwrap()), 242)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load("input/aoc4").unwrap()), 186)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input = load("input/aoc4").unwrap();
        b.iter(|| answer1(&input));
    }
    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input = load("input/aoc4").unwrap();
        b.iter(|| answer2(&input));
    }
}

fn has_all_fields(caps: &Captures) -> bool {
    caps.iter().skip(2).all(|m| m.is_some())
}

fn has_all_valids_fields(caps: &Captures) -> bool {
    const RANGES: [Option<RangeInclusive<u16>>; 8] = [
        // Full match is ignored
        // cid is ignored
        Some(1920u16..=2002), // byr
        Some(2010u16..=2020), // iyr
        Some(2020u16..=2030), // eyr
        Some(150u16..=193),   // hgt_cm
        Some(59u16..=76),     // hgt_in
        None,                 // hcl
        None,                 // ecl
        None,                 // pid
    ];

    caps.iter()
        .skip(2) // Skip full match and cid
        .zip(RANGES.iter())
        .filter(|(m, range)| match m {
            None => false, // No match
            Some(m) => match range {
                None => true, // No range
                Some(range) => range.contains(&m.as_str().parse().unwrap()),
            },
        })
        .count()
        == 7
}

pub fn answer1(passports: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?:(?:cid:(?P<cid>\S+)|byr:(?P<byr>\S+)|iyr:(?P<iyr>\S+)|eyr:(?P<eyr>\S+)|hgt:(?P<hgt>\S+)|hcl:(?P<hcl>\S+)|ecl:(?P<ecl>\S+)|pid:(?P<pid>\S+))\s)+"
        ).unwrap();
    }
    RE.captures_iter(passports).filter(has_all_fields).count()
}

pub fn answer2(passports: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^(?:(?:cid:(?P<cid>\S+)|byr:(?P<byr>(?:19|20)\d{2})|iyr:(?P<iyr>20[12]\d)|eyr:(?P<eyr>20[23]\d)|hgt:(?:(?P<hgt_cm>1\d{2})cm|(?P<hgt_in>\d{2})in)|hcl:(?P<hcl>#[0-9a-f]{6})|ecl:(?P<ecl>amb|blu|brn|gry|grn|hzl|oth)|pid:(?P<pid>\d{9}))\s)+\B").unwrap();
    }
    RE.captures_iter(passports)
        .filter(has_all_valids_fields)
        .count()
}
