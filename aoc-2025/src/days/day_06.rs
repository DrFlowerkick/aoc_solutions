//!day_06.rs

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Operator {
    #[default]
    Add,
    Multiply,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => unreachable!("Only \"+\" and \"*\" in challenge."),
        }
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        Self::from(value.to_string().as_str())
    }
}

#[derive(Default, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

struct ChallengeInput {
    problems: Vec<Problem>,
}

impl ChallengeInput {
    fn parse_input_for_part_1(input: &str) -> Self {
        let num_lines = input.lines().count();
        let numbers: Vec<Vec<u64>> = input
            .lines()
            .take(num_lines - 1)
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect();
        let operators: Vec<Operator> = input
            .lines()
            .nth(num_lines - 1)
            .map(|l| l.split_whitespace().map(Operator::from).collect())
            .unwrap();
        ChallengeInput {
            problems: operators
                .iter()
                .enumerate()
                .map(|(i, o)| Problem {
                    numbers: numbers.iter().map(|n| n[i]).collect(),
                    operator: *o,
                })
                .collect(),
        }
    }
    fn solution(&self) -> u64 {
        self.problems.iter().map(|p| p.solve()).sum()
    }
    fn parse_input_for_part_2(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let len_y = grid.len();
        let mut x = grid[0].len();
        let mut problems: Vec<Problem> = Vec::new();
        let mut problem = Problem::default();
        while x > 0 {
            x -= 1;
            let digits: String = grid.iter().take(len_y - 1).map(|l| l[x]).collect();
            if digits.trim().is_empty() {
                problem = Problem::default();
                continue;
            }
            problem.numbers.push(digits.trim().parse().unwrap());
            if !grid[len_y - 1][x].is_whitespace() {
                problem.operator = grid[len_y - 1][x].into();
                problems.push(problem.clone());
            }
        }
        Self { problems }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_06.txt");

    let result_part1 = ChallengeInput::parse_input_for_part_1(input).solution();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 5_322_004_718_681);

    let result_part2 = ChallengeInput::parse_input_for_part_2(input).solution();
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 9_876_636_978_528);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_06_example.txt");

        let result_part1 = ChallengeInput::parse_input_for_part_1(input).solution();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 4_277_556);

        let result_part2 = ChallengeInput::parse_input_for_part_2(input).solution();
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 3_263_827);

        Ok(())
    }
}
