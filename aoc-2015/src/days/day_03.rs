//!day_03.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashSet;

struct ChallengeInput {
    directions: Vec<Compass>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            directions: value
                .chars()
                .map(|c| match c {
                    '^' => Compass::N,
                    '>' => Compass::E,
                    'v' => Compass::S,
                    '<' => Compass::W,
                    _ => panic!("unknown char"),
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut pos = Point::default();
        seen.insert(pos);
        for dir in self.directions.iter() {
            pos = pos.add(*dir);
            seen.insert(pos);
        }
        seen.len()
    }
    fn solution_part_2(&self) -> usize {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut santa = Point::default();
        let mut robot = santa;
        seen.insert(santa);
        for (i, dir) in self.directions.iter().enumerate() {
            if i & 1 == 0 {
                santa = santa.add(*dir);
                seen.insert(santa);
            } else {
                robot = robot.add(*dir);
                seen.insert(robot);
            }
        }
        seen.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 2_081);

    let result_part2 = challenge.solution_part_2();
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 2_341);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_03_example_part_1.txt");

        let solutions = [2, 4, 2];
        assert_eq!(input.lines().count(), solutions.len());

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_03 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }

    #[test]
    fn test_example_day_03_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_03_example_part_2.txt");

        let solutions = [3, 3, 11];
        assert_eq!(input.lines().count(), solutions.len());

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part2 = example.solution_part_2();
            println!("result day_03 part 2: {result_part2}");
            assert_eq!(result_part2, solution);
        }

        Ok(())
    }
}
