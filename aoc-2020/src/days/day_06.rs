//!day_06.rs

use anyhow::Result;
use std::collections::HashSet;

struct ChallengeInput {
    collected_answers: Vec<HashSet<char>>,
    intersecting_answers: Vec<HashSet<char>>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            collected_answers: value
                .split("\n\n")
                .map(|group| group.lines().flat_map(|l| l.chars()).collect())
                .collect(),
            intersecting_answers: value
                .split("\n\n")
                .filter_map(|group| {
                    group
                        .lines()
                        .fold(None::<HashSet<char>>, |intersection, line| {
                            let line_set: HashSet<char> = line.chars().collect();
                            if let Some(int) = intersection {
                                Some(int.intersection(&line_set).copied().collect())
                            } else {
                                Some(line_set)
                            }
                        })
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.collected_answers.iter().map(|g| g.len()).sum()
    }
    fn solution_part_2(&self) -> usize {
        self.intersecting_answers.iter().map(|g| g.len()).sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_06.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 6_903);

    let result_part2 = challenge.solution_part_2();
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 3_493);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_06_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 11);

        let result_part2 = example.solution_part_2();
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 6);

        Ok(())
    }
}
