//!day_01.rs

use anyhow::Result;

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { input: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> i64 {
        self.input
            .chars()
            .map(|c| match c {
                '(' => 1_i64,
                ')' => -1,
                _ => panic!("unknown char"),
            })
            .sum()
    }
    fn solution_part_2(&self) -> usize {
        let mut floor = 0;
        for (index, step) in self
            .input
            .chars()
            .map(|c| match c {
                '(' => 1_i64,
                ')' => -1,
                _ => panic!("unknown char"),
            })
            .enumerate()
        {
            floor += step;
            if floor == -1 {
                return index + 1;
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, 232);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    assert_eq!(result_part2, 1_783);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_01_example_part_1.txt");

        let solutions = [0, 0, 3, 3, 3, -1, -1, -3, -3];
        assert_eq!(input.lines().count(), solutions.len());

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
        let input = include_str!("../../../../aoc_input/aoc-2015/day_01_example_part_2.txt");

        let solutions = [1, 5];
        assert_eq!(input.lines().count(), solutions.len());

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part2 = example.solution_part_2();
            println!("result day_01 part 2: {result_part2}");
            assert_eq!(result_part2, solution);
        }

        Ok(())
    }
}
