//!day_02.rs

use anyhow::Result;

struct ChallengeInput {
    numbers: Vec<usize>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.split(',').filter_map(|n| n.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, challenge: bool) -> usize {
        let (noun, verb) = if challenge {
            (12, 2)
        } else {
            (self.numbers[1], self.numbers[2])
        };
        self.run_int_code(noun, verb).unwrap()
    }
    fn solution_part_2(&self) -> usize {
        for noun in 0..100 {
            for verb in 0..100 {
                if let Some(int_result) = self.run_int_code(noun, verb)
                    && int_result == 19_690_720
                {
                    return 100 * noun + verb;
                }
            }
        }
        0
    }
    fn run_int_code(&self, noun: usize, verb: usize) -> Option<usize> {
        let mut numbers = self.numbers.clone();
        let mut index = 0;

        numbers[1] = noun;
        numbers[2] = verb;

        while let Some(op_code) = numbers.get(index) {
            let destination_index = *numbers.get(index + 3)?;
            match op_code {
                1 => {
                    *numbers.get_mut(destination_index)? = *numbers
                        .get(*numbers.get(index + 1)?)?
                        + *numbers.get(*numbers.get(index + 2)?)?;
                }
                2 => {
                    *numbers.get_mut(destination_index)? = *numbers
                        .get(*numbers.get(index + 1)?)?
                        * *numbers.get(*numbers.get(index + 2)?)?;
                }
                99 => return Some(numbers[0]),
                _ => panic!("unknown op code"),
            }
            index += 4;
        }

        None
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(true);
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, 2_782_414);

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, 9_820);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_02_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(false);
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, 3_500);

        // no example solution for part 2

        Ok(())
    }
}
