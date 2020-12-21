use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Read;

use crate::common::GroupingCount;
use crate::common::Lines;
use crate::Solution;

fn read_input(input: &mut dyn Read) -> Vec<(Vec<String>, Vec<String>)> {
    let mut foods = Vec::new();

    for line in Lines::new(input) {
        let mut main_split = line.split(" (contains ");

        let ingredients = main_split.next().unwrap();
        let allergens = main_split.next().unwrap().trim_end_matches(')');

        let ingredients = ingredients.split(' ').map(str::to_owned).collect();
        let allergens = allergens.split(", ").map(str::to_owned).collect();

        foods.push((ingredients, allergens));
    }

    foods
}

fn compute_allergens_possible(
    foods: &[(Vec<String>, Vec<String>)],
) -> HashMap<String, Vec<String>> {
    let mut allergen_possible: HashMap<String, Vec<String>> = HashMap::new();

    for (ingredients, allergens) in foods {
        for allergen in allergens {
            match allergen_possible.entry(allergen.clone()) {
                Entry::Occupied(entry) => {
                    entry.into_mut().retain(|a| ingredients.contains(a));
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            }
        }
    }

    allergen_possible
}

#[derive(Default)]
pub struct Day21;

impl Solution for Day21 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let foods = read_input(input);

        let allergens_possible = compute_allergens_possible(&foods);

        let mut ingredient_count = foods
            .iter()
            .map(|(ingredients, _)| ingredients)
            .flatten()
            .grouping_count();

        for possible in allergens_possible.values().flatten() {
            ingredient_count.remove(possible);
        }

        ingredient_count.values().sum::<usize>().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let foods = read_input(input);

        let mut allergen_possible: HashMap<String, Vec<String>> =
            compute_allergens_possible(&foods);

        let mut dangerous = Vec::new();

        while !allergen_possible.is_empty() {
            let mut found = None;

            for (allergen, possible) in &allergen_possible {
                if possible.len() == 1 {
                    found = Some(allergen.clone());
                }
            }

            let allergen = found.expect("Impossible puzzle");
            let entry = allergen_possible
                .remove(&allergen)
                .expect("Invariant violated")
                .pop()
                .unwrap();

            for possible in allergen_possible.values_mut() {
                possible.retain(|p| p != &entry);
            }

            dangerous.push((allergen, entry));
        }

        dangerous.sort();

        dangerous
            .iter()
            .map(|(_, b)| b)
            .fold(String::from(","), |mut a, b| {
                a.push_str(b);
                a.push(',');
                a
            })
            .trim_matches(',')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/21.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day21, 1, SAMPLE, 5);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day21, 2, SAMPLE, "mxmxvkd,sqjhc,fvjkl");
    }
}
