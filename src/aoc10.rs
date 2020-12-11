pub use crate::loaders::file_to as load;
use std::{collections::VecDeque, iter};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc10")), 1856)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc10")), 2314037239808)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<u32> = load("input/aoc10").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<u32> = load("input/aoc10").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

pub fn answer1(adapters: impl Iterator<Item = u32>) -> u32 {
    let mut sorted_adapters: Vec<_> = iter::once(0).chain(adapters).collect();
    sorted_adapters.sort_unstable();

    // Count ones and threes
    let (ones, threes) = sorted_adapters[..]
        .windows(2)
        .fold((0, 0), |mut acc, vals| {
            match vals[1] - vals[0] {
                1 => acc.0 += 1,
                2 => (),
                3 => acc.1 += 1,
                _ => panic!("Unexpected delta"),
            }
            acc
        });
    ones * (threes + 1) // Add one for the last adapter since it is not in the list
}

pub fn answer2(adapters: impl Iterator<Item = u32>) -> u64 {
    /*
    Task is to count the total number of paths between two nodes of a DAG.
    This is not a complex task and is easily described recursively (with cache for speedup).
    It can as easily be done iteratively if we sort the list topologically first.
    */
    // Add first, last, and sort vector
    let mut sorted_adapters: Vec<_> = iter::once(0).chain(adapters).collect();
    sorted_adapters.sort_unstable();
    let end = sorted_adapters.last().unwrap() + 3;
    sorted_adapters.push(end);

    /*
    Iterate list from behind, looking at the items (if possible) before it.
    Use a cache of the closest items to keep track of existing paths. Cache is between 1 and 3 items
    [(adapter-3,) (adapter-2,) adapter-1]
    Since we are iterating backwards over a topologically sorted DAG we are guaranteed to calculate
    each nodes total before we use its value
    */
    iter::once(&sorted_adapters[..2]) // First 2 items
        .chain(iter::once(&sorted_adapters[..3])) // First 3 items
        .chain(sorted_adapters[..].windows(4)) // All other windows of 4 items
        .rev()
        .fold(
            {
                // Cache is initialized with 1 (since the end is only a single node)
                let mut cache = VecDeque::with_capacity(3);
                cache.push_back(1u64);
                cache
            },
            |mut cache, slice| {
                let target_size = cache.pop_back().unwrap();
                let (target, candidates) = slice.split_last().unwrap();
                for (i, source) in candidates.iter().rev().enumerate() {
                    if target - source > 3 {
                        // Not reachable, all subsequent candidates will also be too big
                        break;
                    }
                    // Add the target number of branches to cache
                    if let Some(existing_value) = cache.get_mut(i) {
                        *existing_value += target_size;
                    } else {
                        cache.push_front(target_size);
                    }
                }
                cache
            },
        )[0] // Result is the only number left in the cache
}
