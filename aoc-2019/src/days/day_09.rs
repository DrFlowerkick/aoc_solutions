//!day_09.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;

struct ChallengeInput {
    code: IntCodeComputer,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            code: IntCodeComputer::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut test_run = self.code.clone();
        test_run.run_until_finished(&[1])
    }
    fn solution_part_2(&self) -> i64 {
        let mut test_run = self.code.clone();
        test_run.run_until_finished(&[2])
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 3_638_931_938);

    let result_part2 = challenge.solution_part_2();
    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 86_025);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // day 09 does not provide examples for the challenge. Instead it does provide new
    // int code snippets to test

    #[test]
    fn test_day_05_small_examples() {
        let small_examples =
            include_str!("../../../../aoc_input/aoc-2019/day_09_small_examples.txt");
        let int_code_computers: Vec<IntCodeComputer> =
            small_examples.lines().map(IntCodeComputer::from).collect();
        let mut int_code_computer = int_code_computers[0].clone();
        let mut outs: Vec<String> = Vec::new();
        while let Some(out) = int_code_computer.run_int_code(&[]).unwrap() {
            outs.push(format!("{out}"));
        }
        let merge = outs.join(",");
        assert_eq!(merge, small_examples.lines().next().unwrap());

        let mut int_code_computer = int_code_computers[1].clone();
        let mut outs: Vec<String> = Vec::new();
        while let Some(out) = int_code_computer.run_int_code(&[]).unwrap() {
            outs.push(format!("{out}"));
        }
        assert_eq!(outs.len(), 1);
        assert_eq!(outs[0].chars().count(), 16);

        let mut int_code_computer = int_code_computers[2].clone();
        let mut outs: Vec<i64> = Vec::new();
        while let Some(out) = int_code_computer.run_int_code(&[]).unwrap() {
            outs.push(out);
        }
        assert_eq!(outs.len(), 1);
        assert_eq!(outs[0], 1_125_899_906_842_624);
    }
}
