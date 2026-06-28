//!day_15.rs

use anyhow::Result;
use my_lib::my_algo_collection::collect_all_n_from_m_elements;
use std::{
    collections::{BTreeSet, HashSet},
    ops::Add,
};

#[derive(Clone, Copy, Default)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let mut value_iter = value.split_whitespace().filter_map(|v| {
            if let Some(stripped) = v.strip_suffix(",") {
                stripped
            } else {
                v
            }
            .parse::<i64>()
            .ok()
        });
        Ingredient {
            capacity: value_iter.next().unwrap(),
            durability: value_iter.next().unwrap(),
            flavor: value_iter.next().unwrap(),
            texture: value_iter.next().unwrap(),
            calories: value_iter.next().unwrap(),
        }
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Ingredient {
    fn scale(&self, factor: i64) -> Self {
        Ingredient {
            capacity: self.capacity * factor,
            durability: self.durability * factor,
            flavor: self.flavor * factor,
            texture: self.texture * factor,
            calories: self.calories * factor,
        }
    }
    fn score_part_1(&self) -> i64 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
    fn score_part_2(&self) -> i64 {
        let delta = (500 - self.calories).abs() + 1;
        self.score_part_1() / (delta * delta)
    }
}

struct ChallengeInput {
    ingredients: Vec<Ingredient>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            ingredients: value.lines().map(Ingredient::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        self.find_best_solution(false)
    }
    fn solution_part_2(&self) -> i64 {
        self.find_best_solution(true)
    }
    fn find_best_solution(&self, part_2: bool) -> i64 {
        let mut seen: HashSet<Vec<i64>> = HashSet::new();
        let start_value_for_each_ingredient = 100_i64 / self.ingredients.len() as i64;
        let initial_distribution = vec![start_value_for_each_ingredient; self.ingredients.len()];
        let mut max_value = i64::MIN;
        let indices: Vec<usize> = (0..self.ingredients.len()).collect();
        let switch_indices = collect_all_n_from_m_elements(&indices, 2);
        let mut sorted_queue: BTreeSet<(i64, Vec<i64>)> = BTreeSet::new();
        sorted_queue.insert((
            self.distribution_value(&initial_distribution, part_2),
            initial_distribution,
        ));
        while let Some((value, distribution)) = sorted_queue.pop_last() {
            if seen.insert(distribution.clone()) && value >= max_value {
                max_value = value;
                for (from, to) in switch_indices
                    .iter()
                    .flat_map(|swi| [(swi[0], swi[1]), (swi[1], swi[0])])
                    .filter(|&(from, to)| distribution[from] > 0 && distribution[to] < 100)
                {
                    let mut new_distribution = distribution.clone();
                    new_distribution[from] -= 1;
                    new_distribution[to] += 1;
                    let new_value = self.distribution_value(&new_distribution, part_2);
                    sorted_queue.insert((new_value, new_distribution));
                }
            }
        }
        max_value
    }
    fn distribution_value(&self, distribution: &Vec<i64>, part_2: bool) -> i64 {
        let sum_ingredients = self
            .ingredients
            .iter()
            .zip(distribution)
            .map(|(i, f)| i.scale(*f))
            .fold(Ingredient::default(), |val, ing| val + ing);
        if part_2 {
            sum_ingredients.score_part_2()
        } else {
            sum_ingredients.score_part_1()
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_15.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 21_367_368);

    let result_part2 = challenge.solution_part_2();
    println!("result day_15 part 2: {result_part2}");
    assert_eq!(result_part2, 1_766_400);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_collect_groups() {
        let group = vec![0, 1];
        let possible_groups = collect_all_n_from_m_elements(&group, 2);
        assert_eq!(possible_groups.len(), 1);
        assert_eq!(possible_groups[0], group);

        let group = vec![0, 1, 2, 3];
        let possible_groups = collect_all_n_from_m_elements(&group, 2);
        assert_eq!(possible_groups.len(), 6);
    }

    #[test]
    fn test_example_day_15() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_15_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_15 part 1: {result_part1}");
        assert_eq!(result_part1, 62_842_880);

        let result_part2 = example.solution_part_2();
        println!("result day_15 part 2: {result_part2}");
        assert_eq!(result_part2, 57_600_000);

        Ok(())
    }
}
