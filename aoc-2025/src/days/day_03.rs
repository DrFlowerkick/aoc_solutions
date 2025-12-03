//!day_03.rs

use anyhow::Result;

struct ChallengeInput {
    bat_banks: Vec<String>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            bat_banks: value.lines().map(|l| l.to_string()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut sum_joltage = 0;
        for bb in self.bat_banks.iter() {
            let max_digit = self.find_next_max_digit(bb, 2 - 1);
            sum_joltage += max_digit.parse::<u64>().unwrap();
        }
        sum_joltage
    }
    fn solution_part_2(&self) -> u64 {
        let mut sum_joltage = 0;
        for bb in self.bat_banks.iter() {
            let max_digit = self.find_next_max_digit(bb, 12 - 1);
            sum_joltage += max_digit.parse::<u64>().unwrap();
        }
        sum_joltage
    }
    #[allow(clippy::only_used_in_recursion)]
    fn find_next_max_digit(&self, bb: &str, num_digits: usize) -> String {
        if num_digits == 0 {
            let max_digit = bb.chars().max().unwrap();
            return max_digit.into();
        }
        let max_digit = bb[..bb.len() - num_digits].chars().max().unwrap();
        let index = bb.chars().position(|d| d == max_digit).unwrap();
        let digits = self.find_next_max_digit(&bb[index + 1..], num_digits - 1);
        format!("{}{}", max_digit, digits)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 17_435);

    let result_part2 = challenge.solution_part_2();
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 172_886_048_065_379);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_03_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_03 part 1: {result_part1}");
        assert_eq!(result_part1, 357);

        let result_part2 = example.solution_part_2();
        println!("result day_03 part 2: {result_part2}");
        assert_eq!(result_part2, 3_121_910_778_619);

        Ok(())
    }
}
