//!day_19.rs

use anyhow::Result;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
struct Day19Data {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl From<&str> for Day19Data {
    fn from(value: &str) -> Self {
        let (towels, designs) = value.split_once("\n\n").unwrap();
        Self {
            towels: towels.split(",").map(|t| t.trim().to_string()).collect(),
            designs: designs.lines().map(|t| t.trim().to_string()).collect(),
        }
    }
}

impl Day19Data {
    fn count_possible_designs(&self) -> Vec<usize> {
        let mut cache: HashMap<&str, Option<usize>> = HashMap::new();
        self.designs
            .iter()
            .filter_map(|d| self.check_design(d, &mut cache))
            .collect()
    }
    fn check_design<'a>(
        &'a self,
        design: &'a str,
        cache: &mut HashMap<&'a str, Option<usize>>,
    ) -> Option<usize> {
        if design.is_empty() {
            return Some(1);
        }
        if let Some(cached_design) = cache.get(design) {
            return *cached_design;
        }
        for towel in self.towels.iter() {
            let len_towel = towel.len();
            if len_towel > design.len() {
                continue;
            }
            if towel == &design[..len_towel] {
                if let Some(num_patterns) = self.check_design(&design[len_towel..], cache) {
                    match cache.entry(design) {
                        Entry::Vacant(e) => {
                            e.insert(Some(num_patterns));
                        }
                        Entry::Occupied(mut e) => {
                            if let Some(cached_pattern) = e.get_mut() {
                                *cached_pattern += num_patterns;
                            }
                        }
                    }
                }
            }
        }
        match cache.entry(design) {
            Entry::Vacant(e) => *e.insert(None),
            Entry::Occupied(e) => *e.get(),
        }
    }
}

pub fn day_19() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_19.txt");
    let challenge = Day19Data::from(input);

    let result_part1 = challenge.count_possible_designs();
    println!("result day 19 part 1: {}", result_part1.len());
    assert_eq!(result_part1.len(), 322);

    let result_part2: usize = result_part1.iter().sum();
    println!("result day 19 part 2: {}", result_part2);
    assert_eq!(result_part2, 715_514_563_508_258);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_19_example.txt");
        let challenge = Day19Data::from(input);

        let result_part1 = challenge.count_possible_designs();
        println!("result day 19 part 1: {}", result_part1.len());
        assert_eq!(result_part1.len(), 6);

        let result_part2: usize = result_part1.iter().sum();
        println!("result day 19 part 2: {}", result_part2);
        assert_eq!(result_part2, 16);

        Ok(())
    }
}
