//!day_21.rs

use anyhow::Result;
use std::collections::{HashMap, HashSet};

struct ChallengeInput {
    food_list: Vec<(HashSet<String>, HashSet<String>)>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            food_list: value
                .lines()
                .filter_map(|l| l.split_once(" (contains "))
                .map(|(ing, all)| {
                    (
                        ing.split_whitespace().map(|i| i.to_string()).collect(),
                        all.strip_suffix(')')
                            .unwrap()
                            .split(", ")
                            .map(|i| i.to_string())
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (usize, String) {
        // build up allergens map
        let mut allergens_map: HashMap<String, HashSet<String>> = HashMap::new();
        for (ing, all) in self.food_list.iter() {
            for a in all.iter() {
                allergens_map
                    .entry(a.to_owned())
                    .and_modify(|i| *i = i.intersection(ing).map(|s| s.to_owned()).collect())
                    .or_insert(ing.clone());
            }
        }
        // reduce allergens_map by identified ingredients, which can be mapped exactly to one allergen
        let mut identified_allergen_ing: HashSet<String> = HashSet::new();
        while let Some(ingredient) = allergens_map
            .values()
            .filter(|ing| ing.len() == 1)
            .filter_map(|ing| ing.iter().next())
            .filter(|ing| !identified_allergen_ing.contains(*ing))
            .map(|ing| ing.to_owned())
            .next()
        {
            identified_allergen_ing.insert(ingredient.clone());
            allergens_map
                .values_mut()
                .filter(|ing| ing.len() > 1)
                .for_each(|ing| {
                    ing.remove(&ingredient);
                });
        }
        // number of non allergen ingredients in original list
        let result_part_1 = self
            .food_list
            .iter()
            .flat_map(|(ing, _)| ing.iter())
            .filter(|ing| !identified_allergen_ing.contains(*ing))
            .count();
        // get identified allergens and sort them alphabetically
        let mut allergens: Vec<String> = allergens_map.keys().map(|all| all.to_owned()).collect();
        allergens.sort();
        // get danger list from sorted allergens
        let danger_list: Vec<String> = allergens
            .iter()
            .filter_map(|all| allergens_map.get(all))
            .filter_map(|ing| ing.iter().next())
            .map(|ing| ing.to_owned())
            .collect();
        (result_part_1, danger_list.join(","))
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_21.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, 2_412);

    println!("result day_21 part 2: {result_part2}");
    assert_eq!(
        result_part2,
        "mfp,mgvfmvp,nhdjth,hcdchl,dvkbjh,dcvrf,bcjz,mhnrqp"
    );

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_21() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_21_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_21 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        println!("result day_21 part 2: {result_part2}");
        assert_eq!(result_part2, "mxmxvkd,sqjhc,fvjkl");

        Ok(())
    }
}
