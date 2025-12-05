//!day_05.rs

use anyhow::Result;
use std::collections::VecDeque;

struct ChallengeInput {
    fresh_id_ranges: Vec<(u64, u64)>,
    fruit_ids: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (fresh_id_ranges, fruit_ids) = value.split_once("\n\n").unwrap();
        ChallengeInput {
            fresh_id_ranges: fresh_id_ranges
                .lines()
                .filter_map(|l| {
                    l.split_once('-')
                        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
                })
                .collect(),
            fruit_ids: fruit_ids.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.fruit_ids
            .iter()
            .filter(|f| {
                self.fresh_id_ranges
                    .iter()
                    .any(|(s, e)| (s..=e).contains(f))
            })
            .count()
    }
    fn solution_part_2(&self) -> u64 {
        let mut distinct_fresh_id_ranges: Vec<(u64, u64)> =
            Vec::with_capacity(self.fresh_id_ranges.len() * 2);
        let mut fresh_id_ranges: VecDeque<(u64, u64)> = self.fresh_id_ranges.clone().into();
        'range_loop: while let Some((start, end)) = fresh_id_ranges.pop_front() {
            for &(distinct_start, distinct_end) in distinct_fresh_id_ranges.iter() {
                if end < distinct_start || start > distinct_end {
                    // no overlap
                    continue;
                }
                if start < distinct_start {
                    // keep range before distinct_start
                    fresh_id_ranges.push_back((start, distinct_start - 1));
                }
                if end > distinct_end {
                    // keep range after distinct_end
                    fresh_id_ranges.push_back((distinct_end + 1, end));
                }
                continue 'range_loop;
            }
            distinct_fresh_id_ranges.push((start, end));
        }

        distinct_fresh_id_ranges
            .into_iter()
            .map(|(s, e)| e - s + 1)
            .sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_05.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 643);

    let result_part2 = challenge.solution_part_2();
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 342_018_167_474_526);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_05() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_05_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        let result_part2 = example.solution_part_2();
        println!("result day_05 part 2: {result_part2}");
        assert_eq!(result_part2, 14);

        Ok(())
    }
}
