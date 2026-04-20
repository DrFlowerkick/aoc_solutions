//!day_04.rs

use anyhow::Result;
use std::collections::HashSet;

struct ChallengeInput {
    words: Vec<Vec<String>>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            words: value
                .lines()
                .map(|l| l.split_whitespace().map(Into::into).collect())
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.words
            .iter()
            .filter(|l| {
                let set: HashSet<&str> = l.iter().map(|w| w.as_str()).collect();
                l.len() == set.len()
            })
            .count()
    }
    fn solution_part_2(&self) -> usize {
        self.words
            .iter()
            .filter(|l| {
                let words_with_sorted_chars: HashSet<String> = l.iter().map(|w| {
                    let mut sorted: Vec<char> = w.chars().collect();
                    sorted.sort();
                    sorted.into_iter().collect()
                }).collect();
                l.len() == words_with_sorted_chars.len()
            })
            .count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_04.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 337);

    let result_part2 = challenge.solution_part_2();
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 231);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_04_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        Ok(())
    }

    #[test]
    fn test_example_day_04_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_04_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_04 part 2: {result_part2}");
        assert_eq!(result_part2, 3);

        Ok(())
    }
}
