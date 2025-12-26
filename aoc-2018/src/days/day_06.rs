//!day_06.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::{HashMap, HashSet, VecDeque};

struct ChallengeInput {
    targets: Vec<Point>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            targets: value
                .lines()
                .filter_map(|l| l.split_once(", "))
                .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let min_x = self.targets.iter().map(|t| t.x).min().unwrap();
        let max_x = self.targets.iter().map(|t| t.x).max().unwrap();
        let min_y = self.targets.iter().map(|t| t.y).min().unwrap();
        let max_y = self.targets.iter().map(|t| t.y).max().unwrap();
        let mut areas: HashMap<Point, u64> = HashMap::new();
        'target_loop: for target in self.targets.iter() {
            let mut seen: HashSet<Point> = HashSet::new();
            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(*target);
            while let Some(current) = queue.pop_front() {
                if seen.insert(current) {
                    let d_target = target.delta(current);
                    let d_min_other = self
                        .targets
                        .iter()
                        .filter(|t| *t != target)
                        .map(|t| t.delta(current))
                        .min()
                        .unwrap();
                    if d_target < d_min_other {
                        areas.entry(*target).and_modify(|a| *a += 1).or_insert(1);
                        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                            let next = current.add(offset);
                            if next.x < min_x || next.x > max_x || next.y < min_y || next.y > max_y
                            {
                                // out of map -> infinite area
                                areas.remove(target);
                                continue 'target_loop;
                            }
                            queue.push_back(next);
                        }
                    }
                }
            }
        }

        *areas.values().max().unwrap()
    }
    fn solution_part_2(&self, bound: i64) -> u64 {
        let mean_x = self.targets.iter().map(|t| t.x).sum::<i64>() / self.targets.len() as i64;
        let mean_y = self.targets.iter().map(|t| t.y).sum::<i64>() / self.targets.len() as i64;
        let mut area = 0;
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_back(Point::new(mean_x, mean_y));
        while let Some(current) = queue.pop_front() {
            if seen.insert(current) {
                let sum_d: i64 = self.targets.iter().map(|t| t.delta(current)).sum();
                if sum_d < bound {
                    area += 1;
                    for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                        let next = current.add(offset);
                        queue.push_back(next);
                    }
                }
            }
        }
        area
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_06.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 3_569);

    let result_part2 = challenge.solution_part_2(10_000);
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 48_978);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_06_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 17);

        let result_part2 = example.solution_part_2(32);
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 16);

        Ok(())
    }
}
