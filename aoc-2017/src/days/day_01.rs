//!day_01.rs

use anyhow::Result;

struct ChallengeInput {
    digits: Vec<u32>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            digits: value.chars().filter_map(|c| c.to_digit(10)).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u32 {
        self.calc_sum(1)
    }
    fn solution_part_2(&self) -> u32 {
        self.calc_sum(self.digits.len() / 2)
    }
    fn calc_sum(&self, digit_offset: usize) -> u32 {
        let mut sum = 0;
        for (index, d1) in self.digits.iter().enumerate() {
            let d2_index = (index + digit_offset).rem_euclid(self.digits.len());
            if *d1 == self.digits[d2_index] {
                sum += d1;
            }
        }
        sum
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, 1_343);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_01_example_part_1.txt");
        let solutions = [3, 4, 0, 9];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_01 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }

    #[test]
    fn test_example_day_01_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_01_example_part_2.txt");
        let solutions = [6, 0, 4, 12, 4];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part2 = example.solution_part_2();
            println!("result day_01 part 2: {result_part2}");
            assert_eq!(result_part2, solution);
        }

        Ok(())
    }
}
