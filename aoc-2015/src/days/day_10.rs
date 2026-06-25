//!day_10.rs

use anyhow::Result;

struct ChallengeInput {
    sequence: Vec<char>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            sequence: value.chars().collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, cycles: u64) -> (String, usize) {
        if cycles == 0 {
            return (self.sequence.iter().collect(), self.sequence.len());
        }
        self.one_cycle().solution_part_1(cycles - 1)
    }
    fn solution_part_2(&self) -> usize {
        self.solution_part_1(50).1
    }
    fn one_cycle(&self) -> Self {
        let mut sequence = Vec::with_capacity(self.sequence.len() * 2);
        let mut last_char = ' ';
        let mut count = 0;
        for c in self.sequence.iter() {
            if last_char != ' ' && last_char != *c {
                sequence.push(char::from_digit(count, 10).unwrap());
                sequence.push(last_char);
                count = 0;
            }
            last_char = *c;
            count += 1;
        }
        sequence.push(char::from_digit(count, 10).unwrap());
        sequence.push(last_char);

        ChallengeInput { sequence }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_10.txt");
    let challenge = ChallengeInput::from(input);

    let (_, result_part1) = challenge.solution_part_1(40);
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 360_154);

    let result_part2 = challenge.solution_part_2();
    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 5_103_798);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_10_example.txt");
        let example = ChallengeInput::from(input);

        let (check, result_part1) = example.solution_part_1(5);
        println!("result day_10 part 1: {check} ({result_part1})");
        assert_eq!(check, "312211");
        assert_eq!(result_part1, 6);

        Ok(())
    }
}
