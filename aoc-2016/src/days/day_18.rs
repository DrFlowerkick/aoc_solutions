//!day_18.rs

use anyhow::Result;

struct ChallengeInput {
    tiles: Vec<char>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            tiles: value.chars().collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, lines: u64) -> usize {
        let mut current = self.tiles.clone();
        let mut count = 0;
        for _ in 0..lines {
            count += current.iter().filter(|c| **c == '.').count();
            current = current
                .iter()
                .enumerate()
                .map(|(i, &mid)| {
                    let left = i.checked_sub(1).map_or_else(|| '.', |i| current[i]);
                    let right = *current.get(i + 1).unwrap_or(&'.');
                    match (left, mid, right) {
                        ('^', '^', '.') | ('.', '^', '^') | ('^', '.', '.') | ('.', '.', '^') => {
                            '^'
                        }
                        _ => '.',
                    }
                })
                .collect();
        }
        count
    }
    fn solution_part_2(&self) -> usize {
        self.solution_part_1(400_000)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_18.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(40);
    println!("result day_18 part 1: {result_part1}");
    assert_eq!(result_part1, 1_951);

    let result_part2 = challenge.solution_part_2();
    println!("result day_18 part 2: {result_part2}");
    assert_eq!(result_part2, 20_002_936);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_18_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(10);
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 38);

        Ok(())
    }
}
