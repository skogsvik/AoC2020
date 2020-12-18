pub use crate::loaders::file_to_paragraphs as load;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load("input/aoc16")), 19070)
    }
    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load("input/aoc16")), 161926544831)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let parts: Vec<_> = load("input/aoc16").collect();
        b.iter(|| answer1(parts.iter().cloned()));
    }
    
    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let parts: Vec<_> = load("input/aoc16").collect();
        b.iter(|| answer2(parts.iter().cloned()));
    }
}
type Rule = RangeInclusive<u32>;
type RuleMap = HashMap<String, Vec<Rule>>;
type Ticket = Vec<u32>;

fn parse_ticket(numbers: &str) -> Ticket {
    numbers
        .split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

fn parse_rules_and_tickets(
    mut input: impl Iterator<Item = Vec<String>>,
) -> (RuleMap, Ticket, HashSet<Ticket>) {
    let (rules, your_ticket, nearby_tickets) = input.next_tuple().unwrap();
    let rules = rules
        .into_iter()
        .map(|line| {
            let (name, rules) = line.split_once(": ").unwrap();
            let ranges = rules
                .split(" or ")
                .map(|range| {
                    let (start, stop) = range.split_once('-').unwrap();
                    start.parse().unwrap()..=stop.parse().unwrap()
                })
                .collect();
            (name.to_string(), ranges)
        })
        .collect();

    let your_ticket = parse_ticket(&your_ticket.into_iter().nth(1).unwrap());

    let nearby_tickets = nearby_tickets
        .into_iter()
        .skip(1)
        .map(|line| parse_ticket(&line))
        .collect();

    (rules, your_ticket, nearby_tickets)
}

fn iter_bad_values<'a>(
    rules: &'a HashSet<Rule>,
    tickets: &'a HashSet<Ticket>,
) -> impl Iterator<Item = &'a u32> {
    tickets
        .iter()
        .flatten()
        .filter(move |val| !rules.iter().any(|range| range.contains(*val)))
}

fn iter_good_tickets<'a>(
    rules: &'a HashSet<Rule>,
    tickets: &'a HashSet<Ticket>,
) -> impl Iterator<Item = &'a Ticket> {
    tickets.iter().filter(move |ticket| {
        !ticket
            .iter()
            .any(|val| !rules.iter().any(|range| range.contains(val)))
    })
}

fn deduce_fields(mut all_rules: RuleMap, all_tickets: &HashSet<&Ticket>) -> HashMap<String, usize> {
    let mut field_indices = HashMap::with_capacity(all_rules.len());
    let mut indices: Vec<usize> = (0..all_rules.len()).collect();
    while !indices.is_empty() {
        // Check the number of rules that match all idx of all tickets
        indices.retain(|idx| {
            if let Ok(name) = all_rules
                .iter()
                .filter(|(_, ranges)| {
                    all_tickets
                        .iter()
                        .all(|ticket| ranges.iter().any(|range| range.contains(&ticket[*idx])))
                })
                .exactly_one()
                .map(|(name, _)| name.to_string())
            // Release borrow. Can this be done differently?
            {
                // Only 1 rule, must be the correct field
                println!("column {} must be {}", idx, name);
                all_rules.remove(&name);
                field_indices.insert(name, *idx);
                false
            } else {
                true
            }
        });
    }
    field_indices
}

pub fn answer1(input: impl Iterator<Item = Vec<String>>) -> u32 {
    let (rules, _, tickets) = parse_rules_and_tickets(input);
    let all_rules = rules.into_values().flatten().collect(); // TODO: Speed up by reducing ranges first
    iter_bad_values(&all_rules, &tickets).sum()
}

pub fn answer2(input: impl Iterator<Item = Vec<String>>) -> u64 {
    let (rules, your_ticket, tickets) = parse_rules_and_tickets(input);
    let all_rules = rules.values().flatten().cloned().collect(); // TODO: Speed up by reducing ranges first
    let mut valid_tickets: HashSet<_> = iter_good_tickets(&all_rules, &tickets).collect();
    valid_tickets.insert(&your_ticket);

    let fields = deduce_fields(rules, &valid_tickets);
    println!("Found fields {:?}", &fields);
    fields
        .into_iter()
        .filter_map(|(field, idx)| {
            if field.starts_with("departure") {
                Some(your_ticket[idx] as u64)
            } else {
                None
            }
        })
        .product()
}
