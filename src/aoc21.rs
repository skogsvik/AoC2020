use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
pub use std::fs::read_to_string as load;

pub const DATA: &str = "input/aoc21";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load(DATA).unwrap()), 2614)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(
            answer2(&load(DATA).unwrap()),
            "qhvz,kbcpn,fzsl,mjzrj,bmj,mksmf,gptv,kgkrhg"
        )
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let ingredient_list = load(DATA).unwrap();
        b.iter(|| answer1(&ingredient_list));
    }

    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let ingredient_list = load(DATA).unwrap();
        b.iter(|| answer2(&ingredient_list));
    }
}

type Ingredients = HashSet<String>;

fn parse_allergens<'a>(
    ingredient_list: impl Iterator<Item = &'a str>,
) -> (
    HashMap<String, Ingredients>, // Allergen mapping to possible ingredients
    HashMap<String, Option<String>>, // All ingredients
) {
    let mut all_allergens: HashMap<String, Ingredients> = HashMap::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();

    for line in ingredient_list {
        let (ingredients, allergens) = line[0..line.len() - 1].split_once(" (contains ").unwrap();
        let ingredients: HashSet<_> = ingredients.split(' ').collect();

        all_ingredients.extend(ingredients.iter());
        for allergen in allergens.split(", ") {
            if let Some(potential_ingredients) = all_allergens.get_mut(allergen) {
                potential_ingredients.retain(|ing| ingredients.contains(&ing.as_str()));
            } else {
                all_allergens.insert(
                    allergen.to_string(),
                    ingredients.iter().map(|s| s.to_string()).collect(),
                );
            }
        }
    }

    (
        all_allergens,
        all_ingredients
            .iter()
            .map(|ing| (ing.to_string(), None))
            .collect(),
    )
}

fn deduce_allergens(
    mut allergens: HashMap<String, Ingredients>,
    ingredients: &mut HashMap<String, Option<String>>,
) {
    while !allergens.is_empty() {
        allergens.retain(|allergen, potential| {
            if let Ok(ingredient) = potential
                .iter()
                .filter(|pot_ing| ingredients[pot_ing.as_str()].is_none())
                .exactly_one()
            {
                println!("{} is the allergen of {}", ingredient, allergen);
                *ingredients.get_mut(ingredient).unwrap() = Some(allergen.to_string());
                false
            } else {
                true
            }
        })
    }
}

pub fn answer1(ingredient_list: &str) -> usize {
    let (allergens, ingredients) = parse_allergens(ingredient_list.lines());
    let allergen_ingredients: HashSet<_> = allergens.values().flatten().collect();
    // Building a regex is not the fastest, but it allows won't accidentally match on
    // ingredients which are substrings of others. It would probably be faster the count the
    //occurences in the parsing, but I do not wish to re-write it today
    Regex::new(
        &ingredients
            .keys()
            .filter(|ing| !allergen_ingredients.contains(*ing))
            .join("|"),
    )
    .unwrap()
    .find_iter(ingredient_list)
    .count()
}

pub fn answer2(ingredient_list: &str) -> String {
    let (allergens, mut ingredients) = parse_allergens(ingredient_list.lines());
    deduce_allergens(allergens, &mut ingredients);
    ingredients
        .iter()
        .filter(|(_, opt)| opt.is_some())
        .sorted_by_key(|(_, opt)| opt.as_ref().unwrap())
        .map(|(name, _)| name)
        .join(",")
}
