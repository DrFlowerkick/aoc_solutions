//!day_04.rs

use anyhow::Result;
use rayon::prelude::*;
use std::collections::BTreeSet;

struct ChallengeInput {
    input: String,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            input: value.to_string(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let chunk_size = 500_000;
        let mut start = 0u64;
        loop {
            let end = start + chunk_size;
            let hits: BTreeSet<u64> = (start..end)
                .into_par_iter()
                .filter_map(|count| {
                    let data = format!("{}{}", self.input, count);
                    let digest = md5::compute(data);
                    if digest[0] == 0 && digest[1] == 0 && digest[2] <= 0x0f {
                        Some(count)
                    } else {
                        None
                    }
                })
                .collect();

            if let Some(first) = hits.first() {
                return *first;
            }

            start += chunk_size;
        }
    }
    fn solution_part_2(&self, mut start: u64) -> u64 {
        let chunk_size = 500_000;
        loop {
            let end = start + chunk_size;
            let hits: BTreeSet<u64> = (start..end)
                .into_par_iter()
                .filter_map(|count| {
                    let data = format!("{}{}", self.input, count);
                    let digest = md5::compute(data);
                    if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
                        Some(count)
                    } else {
                        None
                    }
                })
                .collect();

            if let Some(first) = hits.first() {
                return *first;
            }

            start += chunk_size;
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_04.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 254_575);

    let result_part2 = challenge.solution_part_2(result_part1);
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 1_038_736);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_04_example.txt");

        let solutions = [609043, 1048970];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_04 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }
}
