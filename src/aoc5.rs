pub use crate::loaders::file_to_boarding_card_id as load;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load("input/aoc5").collect()), &864)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load("input/aoc5").collect()), Some(739))
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        // I guess this isn't much of a benchmark since all the smarts are in the loader
        let input = load("input/aoc5").collect();
        b.iter(|| answer1(&input));
    }
    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input = load("input/aoc5").collect();
        b.iter(|| answer2(&input));
    }
}

pub fn answer1(boarding_card_ids: &HashSet<u16>) -> &u16 {
    boarding_card_ids.iter().max().unwrap()
}

pub fn answer2(boarding_card_ids: &HashSet<u16>) -> Option<u16> {
    let all_ids: HashSet<u16> = (0..1024).collect();
    all_ids
        .difference(&boarding_card_ids)
        .find(|id| {
            boarding_card_ids.contains(&id.saturating_sub(1))
                && boarding_card_ids.contains(&id.saturating_add(1))
        })
        .cloned()
}
