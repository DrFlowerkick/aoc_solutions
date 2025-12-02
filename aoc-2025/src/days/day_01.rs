//!day_01.rs

use anyhow::Result;

struct ChallengeInput {
    rotations: Vec<(bool, i64)>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            rotations: value
                .lines()
                .map(|n| (n.starts_with('R'), n[1..].parse().unwrap()))
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        self.rotations
            .iter()
            .fold((50, 0), |(mut current_pos, mut count), (dir, rotations)| {
                current_pos = if *dir {
                    current_pos + rotations
                } else {
                    current_pos - rotations
                };
                current_pos = current_pos.rem_euclid(100);
                if current_pos == 0 {
                    count += 1;
                }
                (current_pos, count)
            })
            .1
    }
    fn solution_part_2(&self) -> i64 {
        self.rotations
            .iter()
            .fold((50, 0), |(mut current_pos, mut count), (dir, rotations)| {
                let start_pos = if *dir {
                    current_pos
                } else {
                    (100_i64 - current_pos).rem_euclid(100)
                };
                count += (start_pos + rotations).div_euclid(100);
                current_pos = if *dir {
                    current_pos + rotations
                } else {
                    current_pos - rotations
                };

                (current_pos.rem_euclid(100), count)
            })
            .1
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, 1_034);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    assert_eq!(result_part2, 6_166);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_01_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_01 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        let result_part2 = example.solution_part_2();
        println!("result day_01 part 2: {result_part2}");
        assert_eq!(result_part2, 6);

        Ok(())
    }
}
