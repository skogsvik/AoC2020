pub use crate::loaders::parse_password_req as load;
use std::ops;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc2")), 396)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc2")), 428)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input :Vec<_>= load("input/aoc2").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input :Vec<_>= load("input/aoc2").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

#[derive(Clone)]
pub struct PasswordReq {
    pub range: ops::RangeInclusive<usize>,
    pub character: u8,
}

pub fn answer1(reqs_and_pass: impl Iterator<Item = (crate::aoc2::PasswordReq, String)>) -> usize {
    reqs_and_pass
        .filter(|(req, pass)| {
            req.range
                .contains(&pass.matches(req.character as char).count())
        })
        .count()
}

pub fn answer2(reqs_and_pass: impl Iterator<Item = (crate::aoc2::PasswordReq, String)>) -> usize {
    reqs_and_pass
        .filter(|(req, pass)| {
            let target = req.character as char;
            (pass.chars().nth(*req.range.start()).unwrap() == target)
                ^ (pass.chars().nth(*req.range.end()).unwrap() == target)
        })
        .count()
}
