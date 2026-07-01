//!day_17.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    containers: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            containers: value.lines().filter_map(|v| v.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self, liters: u64) -> (u64, u64) {
        let mut container_count_per_combo: HashMap<u64, u64> = HashMap::new();
        let count = self.recursive_container_count(liters, 0, 0, &mut container_count_per_combo);
        let min_containers_combos = container_count_per_combo
            .iter()
            .min_by_key(|(k, _)| **k)
            .unwrap()
            .1;
        (count, *min_containers_combos)
    }
    fn recursive_container_count(
        &self,
        liters: u64,
        index: usize,
        containers: u64,
        container_count_per_combo: &mut HashMap<u64, u64>,
    ) -> u64 {
        if liters == 0 {
            container_count_per_combo
                .entry(containers)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            return 1;
        }
        if index >= self.containers.len() {
            return 0;
        }
        let mut count = 0;
        for (i, c) in self.containers[index..].iter().enumerate() {
            if *c <= liters {
                count += self.recursive_container_count(
                    liters - c,
                    index + i + 1,
                    containers + 1,
                    container_count_per_combo,
                );
            }
        }
        count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_17.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(150);
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, 1_638);

    println!("result day_17 part 2: {result_part2}");
    assert_eq!(result_part2, 17);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_17() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_17_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2(25);
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 4);

        println!("result day_17 part 2: {result_part2}");
        assert_eq!(result_part2, 3);

        Ok(())
    }
}
