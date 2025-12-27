//!day_12.rs

use anyhow::Result;
use std::collections::BTreeSet;

struct ChallengeInput {
    initial_state: BTreeSet<i64>,
    spreading_rules: BTreeSet<u8>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (initial_state, spreading_rules) = value.split_once("\n\n").unwrap();
        ChallengeInput {
            initial_state: initial_state
                .strip_prefix("initial state: ")
                .unwrap()
                .chars()
                .enumerate()
                .filter_map(|(i, c)| (c == '#').then_some(i as i64))
                .collect(),
            spreading_rules: spreading_rules
                .lines()
                .filter_map(|l| {
                    let (rule, res) = l.split_once(" => ").unwrap();
                    let rule = rule.chars().fold(0, |mut v, c| {
                        v <<= 1;
                        if c == '#' {
                            v += 1
                        };
                        v
                    });
                    if res == "#" { Some(rule) } else { None }
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut current_state = self.initial_state.clone();
        for _ in 0..20 {
            current_state = self.one_generation(current_state);
        }
        current_state.into_iter().sum()
    }
    fn solution_part_2(&self) -> i64 {
        let mut current_state = self.initial_state.clone();
        let cycles = 50_000_000_000_i64;
        let mut offset = 0;
        for g in 0..cycles {
            let min_c = *current_state.first().unwrap();
            /*
            // To debug the pattern, print it. It will stabilize after some cycles.
            // Afterward the pattern will shift with each generation to the left or right
            // for a fix number: the difference between min current state and min new state.
            let max = *current_state.last().unwrap();
            println!("gen: {g} -> min current_state: {min_c}");
            for i in min_c..=max {
                if current_state.contains(&i) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
            */
            let new_state = self.one_generation(current_state.clone());
            let min_n = *new_state.first().unwrap();
            if new_state
                .iter()
                .zip(current_state.iter())
                .all(|(n, c)| *n - min_n == *c - min_c)
            {
                offset = (cycles - g) * (min_n - min_c);
                break;
            }
            current_state = new_state;
        }
        current_state.into_iter().map(|v| v + offset).sum()
    }
    fn one_generation(&self, current_state: BTreeSet<i64>) -> BTreeSet<i64> {
        let mut new_state: BTreeSet<i64> = BTreeSet::new();
        let min = *current_state.iter().min().unwrap() - 2;
        let max = *current_state.iter().max().unwrap() + 2;
        for current in min..=max {
            let pattern: u8 = (-2..=2).fold(0, |mut v, i| {
                v <<= 1;
                if current_state.contains(&(current + i)) {
                    v += 1
                }
                v
            });
            if self.spreading_rules.contains(&pattern) {
                new_state.insert(current);
            }
        }
        new_state
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 1_816);

    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 399_999_999_957);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_12() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_12_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_12 part 1: {result_part1}");
        assert_eq!(result_part1, 325);

        // no example for part 2

        Ok(())
    }
}
