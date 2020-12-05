pub use crate::loaders::file_to_lines as load;
use lazy_static::lazy_static;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc5")), 864)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc5")), Some(739))
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc5").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc5").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

pub fn strings_to_boarding_card_id(
    boarding_cards: impl Iterator<Item = String>,
) -> impl Iterator<Item = u16> {
    /*
    The boarding cards are essentially binary numbers but with characters instead of 1 and 0.
    Multiplying by 8 and adding a 7-bit number is just a leftshift and addition, or in this case:
    just parsing the boarding card as a unsigned 10-bit integer.

    FBFBBFFRLR is simply
    0101100101 or 357 which is the same as its ID
    */
    boarding_cards.map(|line| {
        line.chars().fold(0, |val, pos| match pos {
            'B' | 'R' => (val << 1) + 1,
            'F' | 'L' => val << 1,
            _ => panic!("Unexpected char"),
        })
    })
}

pub fn answer1(boarding_cards: impl Iterator<Item = String>) -> u16 {
    strings_to_boarding_card_id(boarding_cards).max().unwrap()
}

pub fn answer2(boarding_cards: impl Iterator<Item = String>) -> Option<u16> {
    lazy_static! {
        static ref ALL_IDS: HashSet<u16> = (0..1024).collect(); // Every possible 10-bit number
    }
    let existing_ids = strings_to_boarding_card_id(boarding_cards).collect();
    ALL_IDS
        .difference(&existing_ids) // We are looking for missing ids
        .find(|id| {
            // Looking for id where before and after exist
            existing_ids.contains(&id.saturating_sub(1))
                && existing_ids.contains(&id.saturating_add(1))
        })
        .cloned()
}
