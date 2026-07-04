//!day_20.rs

use anyhow::Result;
use my_lib::my_algo_collection::collect_all_divisors;

struct ChallengeInput {
    min_presents: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            min_presents: value.parse().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let mut part_1 = None::<u64>;
        let mut part_2 = None::<u64>;
        for dividend in 1..=self.min_presents {
            let all_divisors = collect_all_divisors(dividend);
            if part_1.is_none() && all_divisors.iter().sum::<u64>() * 10 >= self.min_presents {
                part_1 = Some(dividend);
            }
            if part_2.is_none()
                && all_divisors
                    .iter()
                    .filter(|d| **d * 50 >= dividend)
                    .sum::<u64>()
                    * 11
                    >= self.min_presents
            {
                part_2 = Some(dividend)
            }
            if let Some(p1) = part_1
                && let Some(p2) = part_2
            {
                return (p1, p2);
            }
        }
        (0, 0)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_20.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_20 part 1: {result_part1}");
    assert_eq!(result_part1, 786_240);

    println!("result day_20 part 2: {result_part2}");
    assert_eq!(result_part2, 831_600);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_20_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, _) = example.solution_part_1_and_2();
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 8);

        // no test for part 2

        Ok(())
    }
}
