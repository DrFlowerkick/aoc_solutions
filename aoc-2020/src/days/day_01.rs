//!day_01.rs

use anyhow::Result;

struct ChallengeInput {
    numbers: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.lines().filter_map(|l| l.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        for (index, &n1) in self.numbers.iter().enumerate() {
            for &n2 in self.numbers.iter().skip(index + 1) {
                if n1 + n2 == 2020 {
                    return n1 * n2;
                }
            }
        }
        0
    }
    fn solution_part_2(&self) -> u64 {
        for (index, &n1) in self.numbers.iter().enumerate() {
            for &n2 in self.numbers.iter().skip(index + 1) {
                for &n3 in self.numbers.iter().skip(index + 2) {
                    if n1 + n2 + n3 == 2020 {
                        return n1 * n2 * n3;
                    }
                }
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_01_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_01 part 1: {result_part1}");
        assert_eq!(result_part1, 514_579);

        let result_part2 = example.solution_part_2();
        println!("result day_01 part 2: {result_part2}");
        assert_eq!(result_part2, 241_861_950);

        Ok(())
    }
}
