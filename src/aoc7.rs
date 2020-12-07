pub use crate::loaders::file_to_lines as load;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc7")), 274)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc7")), 158730)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc7").collect();
        b.iter(|| answer1(input.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let input: Vec<_> = load("input/aoc7").collect();
        b.iter(|| answer2(input.iter().cloned()));
    }
}

#[derive(Debug, Eq)]
struct Recipe {
    color: String,
    quantity: u32,
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Recipe) -> bool {
        self.color == other.color
    }
}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

impl Borrow<str> for Recipe {
    fn borrow(&self) -> &str {
        self.color.as_str()
    }
}

fn parse_rules(rules: impl Iterator<Item = String>) -> HashMap<String, HashSet<Recipe>> {
    lazy_static! {
        static ref BAG: Regex = Regex::new(r"(\d+) ([ \w]+) bags?[,.]").unwrap();
    }
    rules
        .map(|rule| {
            let (color, contents) = rule.split_once(" bags contain ").unwrap();
            let recipes: HashSet<_> = BAG
                .captures_iter(contents)
                .map(|caps| Recipe {
                    quantity: caps[1].parse().unwrap(),
                    color: caps[2].to_owned(),
                })
                .collect();
            (color.to_owned(), recipes)
        })
        .collect()
}

fn can_create(rules: &HashMap<String, HashSet<Recipe>>, source: &str, target: &str) -> bool {
    // This can be made faster by caching already visited nodes
    let recipes = &rules[source];
    recipes.contains(target)
        || recipes
            .iter()
            .any(|rule| can_create(rules, &rule.color, target))
}

fn count_content(rules: &HashMap<String, HashSet<Recipe>>, source: &str) -> u32 {
    // This can be made faster by caching already visited nodes
    rules[source]
        .iter()
        .map(|rec| rec.quantity * (count_content(rules, &rec.color) + 1))
        .sum()
}

pub fn answer1(rules: impl Iterator<Item = String>) -> usize {
    let rules = parse_rules(rules);
    rules
        .keys()
        .filter(|source| can_create(&rules, source, "shiny gold"))
        .count()
}

pub fn answer2(rules: impl Iterator<Item = String>) -> u32 {
    count_content(&parse_rules(rules), "shiny gold")
}
