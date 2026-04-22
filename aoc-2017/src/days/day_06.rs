//!day_06.rs

use anyhow::Result;
use std::collections::HashSet;

struct ChallengeInput<const N: usize> {
    banks: [u32; N],
}

impl<const N: usize> From<&str> for ChallengeInput<N> {
    fn from(value: &str) -> Self {
        let mut out = ChallengeInput { banks: [0; N] };
        for (index, blocks) in value
            .split_whitespace()
            .filter_map(|b| b.parse().ok())
            .enumerate()
        {
            out.banks[index] = blocks;
        }
        out
    }
}

impl<const N: usize> ChallengeInput<N> {
    fn solution_part_1(&mut self) -> u64 {
        self.count_cycles()
    }
    fn solution_part_2(&mut self) -> u64 {
        self.count_cycles()
    }
    fn count_cycles(&mut self) -> u64 {
        let mut seen: HashSet<[u32; N]> = HashSet::new();
        let mut counter = 0;
        while seen.insert(self.banks) {
            counter += 1;
            let mut max_blocks = *self.banks.iter().max().unwrap();
            let mut index = self.banks.iter().position(|b| *b == max_blocks).unwrap();
            self.banks[index] = 0;
            while max_blocks > 0 {
                index = (index + 1).rem_euclid(N);
                self.banks[index] += 1;
                max_blocks -= 1;
            }
        }
        counter
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_06.txt");
    let mut challenge = ChallengeInput::<16>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 6_681);

    let result_part2 = challenge.solution_part_2();
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 2_392);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_06_example.txt");
        let mut example = ChallengeInput::<4>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        let result_part2 = example.solution_part_2();
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 4);

        Ok(())
    }
}
