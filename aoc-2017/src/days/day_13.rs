//!day_13.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    firewall: HashMap<u64, u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut firewall = HashMap::new();
        for line in value.lines() {
            let (depth, range) = line.split_once(": ").unwrap();
            let depth = depth.parse().unwrap();
            let range = range.parse().unwrap();
            firewall.insert(depth, range);
        }
        ChallengeInput { firewall }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.firewall
            .iter()
            .filter(|(depth, range)| depth.is_multiple_of((**range - 1) * 2))
            .map(|(depth, range)| depth * range)
            .sum()
    }
    fn solution_part_2(&self) -> u64 {
        for delay in 0..u64::MAX {
            if self
                .firewall
                .iter()
                .all(|(depth, range)| !(delay + depth).is_multiple_of((range - 1) * 2))
            {
                return delay;
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_13.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 2_164);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, 3_861_798);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_13_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, 24);

        let result_part2 = example.solution_part_2();
        println!("result day_13 part 2: {result_part2}");
        assert_eq!(result_part2, 10);

        Ok(())
    }
}
