//!day_02.rs

use anyhow::Result;

struct ChallengeInput {
    digits: Vec<Vec<u64>>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            digits: value
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .filter_map(|d| d.parse().ok())
                        .collect()
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.digits
            .iter()
            .map(|l| {
                let min = l.iter().min().unwrap();
                let max = l.iter().max().unwrap();
                *max - *min
            })
            .sum()
    }
    fn solution_part_2(&self) -> u64 {
        self.digits
            .iter()
            .map(|l| {
                let mut quotient = 0;
                'outer: for dividend in l.iter() {
                    for divisor in l.iter().filter(|d| *d < dividend) {
                        if dividend.is_multiple_of(*divisor) {
                            quotient = *dividend / divisor;
                            break 'outer;
                        }
                    }
                }
                quotient
            })
            .sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, 32_121);

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, 197);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_02_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, 18);

        Ok(())
    }

    #[test]
    fn test_example_day_02_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_02_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_02 part 2: {result_part2}");
        assert_eq!(result_part2, 9);

        Ok(())
    }
}
