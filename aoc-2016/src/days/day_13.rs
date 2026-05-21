//!day_13.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{HashSet, VecDeque};

struct ChallengeInput {
    seed: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            seed: value.parse().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, target: Point) -> u64 {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, u64)> = VecDeque::new();
        queue.push_back(((1, 1).into(), 0));
        while let Some((pos, distance)) = queue.pop_front() {
            if pos == target {
                return distance;
            }
            if seen.insert(pos) {
                for next_pos in Compass::cardinals()
                    .iter()
                    .map(|c| pos.add(*c))
                    .filter(|n| n.x >= 0 && n.y >= 0 && self.is_open_space(n))
                {
                    queue.push_back((next_pos, distance + 1));
                }
            }
        }
        0
    }
    fn solution_part_2(&self) -> usize {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, u64)> = VecDeque::new();
        let max_distance = 50;
        queue.push_back(((1, 1).into(), 0));
        while let Some((pos, distance)) = queue.pop_front() {
            if distance > max_distance {
                continue;
            }
            if seen.insert(pos) {
                for next_pos in Compass::cardinals()
                    .iter()
                    .map(|c| pos.add(*c))
                    .filter(|n| n.x >= 0 && n.y >= 0 && self.is_open_space(n))
                {
                    queue.push_back((next_pos, distance + 1));
                }
            }
        }
        seen.len()
    }
    fn is_open_space(&self, pos: &Point) -> bool {
        let x = pos.x as u64;
        let y = pos.y as u64;
        let check = x * x + 3 * x + 2 * x * y + y + y * y + self.seed;
        check.count_ones() & 1 == 0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_13.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1((31, 39).into());
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 96);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, 141);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_13_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1((7, 4).into());
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, 11);

        Ok(())
    }
}
