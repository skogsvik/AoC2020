pub use crate::loaders::file_to_paragraphs as load;
use itertools::Itertools;
use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

pub const DATA: &str = "input/aoc22";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 35299)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 33266)
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let decks: Vec<_> = load(DATA).collect();
        b.iter(|| answer1(decks.iter().cloned()));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let decks: Vec<_> = load(DATA).collect();
        b.iter(|| answer2(decks.iter().cloned()));
    }
}

type Deck = VecDeque<u32>;

fn parse_decks(decks: impl Iterator<Item = Vec<String>>) -> (Deck, Deck) {
    decks
        .map(|deck| {
            deck.iter()
                .skip(1)
                .map(|line| line.parse().unwrap())
                .collect()
        })
        .collect_tuple()
        .unwrap()
}

fn recursive_combat(deck1: &mut Deck, deck2: &mut Deck, mut previous_games: HashSet<u64>) -> bool // True if deck1 wins, false if deck2 wins
{
    loop {
        let mut hasher = DefaultHasher::new();
        deck1.hash(&mut hasher);
        deck2.hash(&mut hasher);
        if !previous_games.insert(hasher.finish()) {
            return true;
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        let deck1_won_round = if card1 as usize <= deck1.len() && card2 as usize <= deck2.len() {
            recursive_combat(
                &mut deck1.iter().take(card1 as usize).cloned().collect(),
                &mut deck2.iter().take(card2 as usize).cloned().collect(),
                HashSet::new(),
            )
        } else {
            card1 > card2
        };
        if deck1_won_round {
            deck1.push_back(card1);
            deck1.push_back(card2);
            if deck2.is_empty() {
                return true;
            }
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
            if deck1.is_empty() {
                return false;
            }
        }
    }
}

fn combat(mut deck1: Deck, mut deck2: Deck) -> Deck {
    while !(deck1.is_empty() || deck2.is_empty()) {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    if deck1.is_empty() {
        deck2
    } else {
        deck1
    }
}

pub fn answer1(decks: impl Iterator<Item = Vec<String>>) -> usize {
    let (deck1, deck2) = parse_decks(decks);

    combat(deck1, deck2)
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card as usize)
        .sum()
}
pub fn answer2(decks: impl Iterator<Item = Vec<String>>) -> usize {
    let (mut deck1, mut deck2) = parse_decks(decks);

    if recursive_combat(&mut deck1, &mut deck2, HashSet::new()) {
        deck1
    } else {
        deck2
    }
    .into_iter()
    .rev()
    .enumerate()
    .map(|(i, card)| (i + 1) * card as usize)
    .sum()
}
