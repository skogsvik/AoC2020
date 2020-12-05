pub use crate::loaders::file_to_lines as load;
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
        let input: Vec<_> = load("input/aoc2").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc2").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

#[derive(Clone)]
pub struct PasswordReq {
    pub range: ops::RangeInclusive<usize>,
    pub character: char,
}

pub fn parse_req_and_password(
    password_candidates: impl Iterator<Item = String>,
) -> impl Iterator<Item = (PasswordReq, String)> {
    password_candidates.map(|line| {
        // Splitting instead of regex since it is more straightforward
        let (req, password) = line.split_once(':').unwrap();
        let (range, character) = req.split_once(' ').unwrap();
        let (start, stop) = range.split_once('-').unwrap();

        let password_req = PasswordReq {
            range: start.parse().unwrap()..=stop.parse().unwrap(),
            character: character.chars().next().unwrap(), // Converting &str to char is gross
        };
        (password_req, password.to_owned())
    })
}

pub fn answer1(password_candidates: impl Iterator<Item = String>) -> usize {
    parse_req_and_password(password_candidates)
        .filter(|(req, pass)| req.range.contains(&pass.matches(req.character).count()))
        .count()
}

pub fn answer2(password_candidates: impl Iterator<Item = String>) -> usize {
    parse_req_and_password(password_candidates)
        .filter(|(req, pass)| {
            (pass.chars().nth(*req.range.start()).unwrap() == req.character)
                ^ (pass.chars().nth(*req.range.end()).unwrap() == req.character)
        })
        .count()
}
