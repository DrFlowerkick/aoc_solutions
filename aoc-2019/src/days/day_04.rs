//!day_04.rs

use anyhow::Result;

struct ChallengeInput {
    lower_bound: u64,
    upper_bound: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (lower_bound, upper_bound) = if let Some((lb, ub)) = value.split_once('-') {
            (lb.parse().unwrap(), ub.parse().unwrap())
        } else {
            let value: u64 = value.parse().unwrap();
            (value, value)
        };
        ChallengeInput {
            lower_bound,
            upper_bound,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        (self.lower_bound..=self.upper_bound)
            .filter(|v| self.is_valid_part_1(*v))
            .count()
    }
    fn solution_part_2(&self) -> usize {
        (self.lower_bound..=self.upper_bound)
            .filter(|v| self.is_valid_part_2(*v))
            .count()
    }
    fn is_valid_part_1(&self, value: u64) -> bool {
        let val_string = format!("{value}");
        let mut double = false;
        for (c1, c2) in val_string.chars().zip(val_string.chars().skip(1)) {
            if c2 < c1 {
                return false;
            }
            double |= c1 == c2;
        }
        double
    }
    fn is_valid_part_2(&self, value: u64) -> bool {
        let val_string = format!("{value}");
        let mut double = false;
        let mut count = 1;
        for (c1, c2) in val_string.chars().zip(val_string.chars().skip(1)) {
            if c2 < c1 {
                return false;
            }
            if c1 != c2 {
                if count == 2 {
                    double = true;
                }
                count = 1;
            } else {
                count += 1;
            }
        }
        double || count == 2
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_04.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 1_890);

    let result_part2 = challenge.solution_part_2();
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 1_277);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04() -> Result<()> {
        let single_examples = include_str!("../../../../aoc_input/aoc-2019/day_04_example.txt");

        let (solution_part_1, solution_part_2) = (6, 3);

        let mut result_part1 = 0;
        let mut result_part2 = 0;
        for input in single_examples.lines() {
            let example = ChallengeInput::from(input);

            result_part1 += example.solution_part_1();
            result_part2 += example.solution_part_2();
        }

        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, solution_part_1);

        println!("result day_04 part 2: {result_part2}");
        assert_eq!(result_part2, solution_part_2);

        Ok(())
    }
}
