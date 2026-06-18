//!day_08.rs

use anyhow::Result;

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(input: &'a str) -> Self {
        ChallengeInput { input }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let mut num_chars = 0;
        let mut num_chars_in_memory = 0;
        let mut num_escaped_chars = 0;
        let mut escape = false;
        let mut hex = None::<u8>;

        for line in self.input.lines() {
            // part 2: add new quotes at start and end
            num_escaped_chars += 2;
            for c in line.chars() {
                num_chars += 1;
                if let Some(h) = &mut hex {
                    *h += 1;
                    if *h == 2 {
                        num_chars_in_memory += 1;
                        hex = None;
                    }
                } else if escape && c == 'x' {
                    hex = Some(0);
                    escape = false;
                } else if escape {
                    num_chars_in_memory += 1;
                    escape = false;
                } else if c == '\\' {
                    escape = true;
                } else if c == '"' {
                    // do nothing, because these are the quotes at start and end of line
                } else {
                    num_chars_in_memory += 1;
                }

                // part 2
                if c == '"' || c == '\\' {
                    num_escaped_chars += 2;
                } else {
                    num_escaped_chars += 1;
                }
            }
        }
        (
            num_chars - num_chars_in_memory,
            num_escaped_chars - num_chars,
        )
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_08.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 1_333);

    println!("result day_08 part 2: {result_part2}");
    assert_eq!(result_part2, 2_046);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_08_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 12);

        println!("result day_08 part 2: {result_part2}");
        assert_eq!(result_part2, 19);

        Ok(())
    }
}
