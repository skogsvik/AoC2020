pub use crate::loaders::file_to_paragraphs as load;
use std::collections::HashSet;

pub const DATA: &str = "input/aoc6";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 6680)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 3117)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load(DATA).collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

pub fn answer1(passenger_group: impl Iterator<Item = Vec<String>>) -> usize {
    // Return the sum of the number of unique letters per group
    passenger_group
        .map(|responses| {
            // Collect all the unique letters within the group.
            responses
                .iter()
                .flat_map(|passenger| passenger.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn answer2(passenger_group: impl Iterator<Item = Vec<String>>) -> usize {
    // Return the sum of the number of letters common to all passengers per group
    passenger_group
        .map(|responses| {
            responses
                .iter()
                // Collect characters per passenger
                .map(|passenger| passenger.chars().collect::<HashSet<_>>())
                // Get the intersection of all sets
                .fold_first(|mut group, pass| {
                    group.retain(|ans| pass.contains(ans));
                    group
                })
                .unwrap()
                .len()
        })
        .sum()
}
