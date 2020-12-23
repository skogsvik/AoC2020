pub use crate::loaders::file_to_lines as load;
use std::collections::{HashSet, VecDeque};

pub const DATA: &str = "input/aoc8";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 1801)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 2060)
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

fn string_to_code<'a>(
    code: impl Iterator<Item = String> + 'a,
) -> impl Iterator<Item = (String, i32)> {
    code.map(|line| {
        let (com, par) = line.split_once(' ').unwrap();
        (com.to_string(), par.parse().unwrap())
    })
}

pub fn run_machine(code: &[(String, i32)]) -> Result<i32, (i32, HashSet<usize>)> {
    let mut visited_pointers = HashSet::new();
    let mut point = 0usize;
    let mut acc = 0;
    loop {
        if point >= code.len() {
            return Ok(acc);
        }
        if !visited_pointers.insert(point) {
            return Err((acc, visited_pointers));
        }
        let (com, par) = &code[point];
        if com == "jmp" {
            point = (point as i32 + par) as usize; // Not sure what the best way of doing this is
            continue;
        } else if com == "acc" {
            acc += par
        }
        point += 1;
    }
}

struct CodeFixer {
    bad_code: Vec<(String, i32)>,
    swaps: VecDeque<usize>,
}

impl CodeFixer {
    fn new(code: impl Iterator<Item = (String, i32)>) -> Self {
        let bad_code: Vec<_> = code.collect();
        let swaps = run_machine(&bad_code)
            .unwrap_err()
            .1
            .drain()
            .filter(|i| &bad_code[*i].0 != "acc") // We only care for the nops and jmps
            .collect();
        Self { bad_code, swaps }
    }
}

impl Iterator for CodeFixer {
    type Item = Vec<(String, i32)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fixed_code = self.bad_code.to_vec(); // Copy code

        let swap = self.swaps.pop_front()?;
        fixed_code[swap].0 = match &fixed_code[swap].0[..] {
            "jmp" => "nop".to_string(),
            "nop" => "jmp".to_string(),
            _ => panic!(),
        };
        Some(fixed_code)
    }
}

pub fn answer1(code: impl Iterator<Item = String>) -> i32 {
    run_machine(&string_to_code(code).collect::<Vec<_>>())
        .unwrap_err()
        .0
}

pub fn answer2(code: impl Iterator<Item = String>) -> i32 {
    // This could be faster if we branched in the machine instead of re-running it each time
    CodeFixer::new(string_to_code(code))
        .find_map(|candidate| run_machine(&candidate).ok())
        .unwrap()
}
