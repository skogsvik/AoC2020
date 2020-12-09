pub use crate::loaders::file_to as load;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(
            answer1(&load("input/aoc9").collect::<Vec<_>>(), 25),
            167829540
        )
    }

    #[test]
    fn test_answer2() {
        assert_eq!(
            answer2(&load("input/aoc9").collect::<Vec<_>>(), &167829540),
            28045630
        )
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc9").collect();
        b.iter(|| answer1(&input, 25));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc9").collect();
        b.iter(|| answer2(&input, &167829540));
    }
}

pub fn answer1(code: &[u64], window_size: usize) -> u64 {
    code.windows(window_size + 1)
        .find_map(|nums| {
            let preamble = &nums[..window_size];
            let current = nums[window_size];
            if preamble
                .iter()
                .any(|&n1| current > n1 && current != n1 * 2 && preamble.contains(&(current - n1)))
            {
                return None;
            }
            // Current is not the sum of 2 distinct numbers in the previous window_size numbers
            Some(current)
        })
        .unwrap()
}

pub fn answer2(code: &[u64], target: &u64) -> u64 {
    // Find the window of the data which sums up to target
    let range = (2..) // Check every window size, this could probably use an heuristic
        .find_map(|size| {
            code.windows(size).find_map(|nums| {
                if nums.iter().sum::<u64>() == *target {
                    return Some(nums);
                }
                None
            })
        })
        .unwrap();
    *range.iter().min().unwrap() + *range.iter().max().unwrap()
}
