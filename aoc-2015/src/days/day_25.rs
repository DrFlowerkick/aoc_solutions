//!day_25.rs

use anyhow::Result;

struct ChallengeInput {
    row: u64,
    column: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let stripped = value
            .strip_prefix(
                "To continue, please consult the code grid in the manual.  Enter the code at row ",
            )
            .unwrap();
        let (row, column) = stripped.split_once(", column ").unwrap();
        let column = column.strip_suffix(".").unwrap();
        ChallengeInput {
            row: row.parse().unwrap(),
            column: column.parse().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let code_number = self.get_code_number();
        let mut code = 20_151_125;
        for _ in 1..code_number {
            code *= 252_533;
            code %= 33_554_393;
        }

        code
    }
    fn get_code_number(&self) -> u64 {
        // get code number in first column
        let mut prev_value = 1;
        let mut code_number = 0;
        for row in 1..=self.row {
            code_number += prev_value;
            prev_value = row;
        }
        // get code number in column
        for column in 1..self.column {
            code_number += self.row + column;
        }
        code_number
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_25.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_25 part 1: {result_part1}");
    assert_eq!(result_part1, 9_132_360);

    // no part 2 on day 25

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_code_number() {
        let twelve = ChallengeInput { row: 4, column: 2 };
        assert_eq!(twelve.get_code_number(), 12);

        let nineteen = ChallengeInput { row: 3, column: 4 };
        assert_eq!(nineteen.get_code_number(), 19);
    }

    #[test]
    fn test_example_day_25() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_25_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_25 part 1: {result_part1}");
        assert_eq!(result_part1, 27_995_004);

        Ok(())
    }
}
