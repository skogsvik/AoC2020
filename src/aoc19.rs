pub use crate::loaders::file_to_paragraphs as load;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub const DATA: &str = "input/aoc19";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 134)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 377)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let parts: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(parts.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let parts: Vec<_> = load(DATA).collect();
        b.iter(|| answer2(parts.iter().cloned()));
    }
}
type RuleMap = HashMap<u32, String>;

fn parse_rules_and_codes(
    mut input: impl Iterator<Item = Vec<String>>,
) -> (RuleMap, HashSet<String>) {
    let (rule_def, codes) = input.next_tuple().unwrap();
    let mut rules = HashMap::with_capacity(rule_def.len());
    let rule_def = rule_def
        .iter()
        .map(|rule| {
            let (num, def) = rule.split_once(": ").unwrap();
            (num.parse::<u32>().unwrap(), def.to_string())
        })
        .collect();
    parse_rule(0, &rule_def, &mut rules);

    let codes = codes.into_iter().collect();
    (rules, codes)
}

fn parse_rule(rule: u32, rule_def: &RuleMap, rules: &mut RuleMap) {
    if rules.contains_key(&rule) {
        return;
    }
    let regex = match rule_def[&rule]
        .split('|')
        .map(|rule_set| {
            rule_set
                .trim()
                .split(' ')
                .map(|n| {
                    if let Ok(n) = n.parse() {
                        parse_rule(n, rule_def, rules);
                        rules[&n].to_string()
                    } else {
                        n.chars().nth(1).unwrap().to_string()
                    }
                })
                .join("")
        })
        .exactly_one()
    {
        Ok(r) => r,
        Err(mut it) => format!("(?:{})", it.join("|")),
    };
    rules.insert(rule, regex);
}

pub fn answer1(input: impl Iterator<Item = Vec<String>>) -> usize {
    let (rules, codes) = parse_rules_and_codes(input);
    let re = Regex::new(&format!("^{}$", rules[&0])).unwrap();
    codes.iter().filter(|code| re.is_match(code)).count()
}

pub fn answer2(input: impl Iterator<Item = Vec<String>>) -> usize {
    let (rules, codes) = parse_rules_and_codes(input);
    let full = Regex::new(&format!("^((?:{})+)((?:{})+)$", rules[&42], rules[&31])).unwrap();
    let first = Regex::new(&rules[&42]).unwrap();
    let second = Regex::new(&rules[&31]).unwrap();
    codes
        .iter()
        .filter(|code| {
            if let Some(cap) = full.captures(code) {
                first.find_iter(&cap[1]).count() > second.find_iter(&cap[2]).count()
            } else {
                false
            }
        })
        .count()
}
