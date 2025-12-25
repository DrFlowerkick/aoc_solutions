//!day_01.rs

use anyhow::Result;
use std::collections::HashSet;

struct ChallengeInput {
    numbers: Vec<i64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        self.numbers.iter().sum()
    }
    fn solution_part_2(&self) -> i64 {
        let mut seen: HashSet<i64> = HashSet::new();
        let mut frequency = 0;
        for number in self.numbers.iter().cycle() {
            if seen.contains(&frequency) {
                return frequency;
            }
            seen.insert(frequency);
            frequency += number;
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, 423);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    assert_eq!(result_part2, 61_126);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_01_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_01 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        let result_part2 = example.solution_part_2();
        println!("result day_01 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
