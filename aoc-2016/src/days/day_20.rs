//!day_20.rs

use anyhow::Result;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Range {
    low: u64,
    high: u64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (low, high) = value.split_once('-').unwrap();
        let low: u64 = low.parse().unwrap();
        let high: u64 = high.parse().unwrap();
        Range {
            low: low.min(high),
            high: low.max(high),
        }
    }
}

impl Range {
    fn block_range(&self, block: &Range) -> Option<Vec<Range>> {
        let mut unblocked: Vec<Range> = Vec::new();
        if self.high < block.low || self.low > block.high {
            return None;
        }
        if self.low < block.low && self.high >= block.low {
            unblocked.push(Range {
                low: self.low,
                high: block.low - 1,
            });
        }
        if self.high > block.high && self.low <= block.high {
            unblocked.push(Range {
                low: block.high + 1,
                high: self.high,
            });
        }
        Some(unblocked)
    }
}

struct ChallengeInput {
    ranges: Vec<Range>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            ranges: value.lines().map(Range::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self, max: u64) -> (u64, u64) {
        let mut unblocked_ranges: Vec<Range> = Vec::new();
        let mut queue: VecDeque<Range> = VecDeque::new();
        queue.push_back(Range { low: 0, high: max });
        while let Some(current) = queue.pop_front() {
            if let Some(unchecked_ranges) = self
                .ranges
                .iter()
                .filter_map(|block| current.block_range(block))
                .next()
            {
                queue.extend(unchecked_ranges);
            } else {
                unblocked_ranges.push(current);
            }
        }
        (
            unblocked_ranges.iter().map(|r| r.low).min().unwrap(),
            unblocked_ranges.iter().map(|r| 1 + r.high - r.low).sum(),
        )
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_20.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(4_294_967_295);
    println!("result day_20 part 1: {result_part1}");
    assert_eq!(result_part1, 4_793_564);

    println!("result day_20 part 2: {result_part2}");
    assert_eq!(result_part2, 146);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_20_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2(9);
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        println!("result day_20 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
