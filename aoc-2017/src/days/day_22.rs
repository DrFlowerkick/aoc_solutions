//!day_22.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{HashMap, HashSet};

enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

struct ChallengeInput {
    map: HashSet<Point>,
    start: Point,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut map = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in value.lines().enumerate() {
            let y = y as i64;
            max_y = max_y.max(y);
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                max_x = max_x.max(x);
                if c == '#' {
                    map.insert(Point::new(x, y));
                }
            }
        }
        ChallengeInput {
            map,
            start: Point::new(max_x / 2, max_y / 2),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut map = self.map.clone();
        let mut pos = self.start;
        let mut dir = Compass::N;
        let mut count = 0;
        for _ in 0..10_000 {
            dir = if map.contains(&pos) {
                map.remove(&pos);
                dir.clockwise().clockwise()
            } else {
                count += 1;
                map.insert(pos);
                dir.counterclockwise().counterclockwise()
            };
            pos = pos.add(dir);
        }
        count
    }
    fn solution_part_2(&self) -> u64 {
        let mut map: HashMap<Point, State> =
            self.map.iter().map(|p| (*p, State::Infected)).collect();
        let mut pos = self.start;
        let mut dir = Compass::N;
        let mut count = 0;
        for _ in 0..10_000_000 {
            dir = match map.get(&pos) {
                None | Some(State::Clean) => {
                    map.entry(pos)
                        .and_modify(|v| *v = State::Weakened)
                        .or_insert(State::Weakened);
                    dir.counterclockwise().counterclockwise()
                }
                Some(State::Weakened) => {
                    count += 1;
                    map.entry(pos).and_modify(|v| *v = State::Infected);
                    dir
                }
                Some(State::Infected) => {
                    map.entry(pos).and_modify(|v| *v = State::Flagged);
                    dir.clockwise().clockwise()
                }
                Some(State::Flagged) => {
                    map.entry(pos).and_modify(|v| *v = State::Clean);
                    dir.flip()
                }
            };
            pos = pos.add(dir);
        }
        count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_22.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 5_256);

    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 2_511_345);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_22_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_22 part 1: {result_part1}");
        assert_eq!(result_part1, 5_587);

        let result_part2 = example.solution_part_2();
        println!("result day_22 part 2: {result_part2}");
        assert_eq!(result_part2, 2_511_944);

        Ok(())
    }
}
