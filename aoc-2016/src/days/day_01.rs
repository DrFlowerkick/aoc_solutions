//!day_01.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
    Left(i64),
    Right(i64),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let distance: i64 = value[1..].parse().unwrap();
        match &value[..1] {
            "L" => Instruction::Left(distance),
            "R" => Instruction::Right(distance),
            _ => panic!("bad Instruction"),
        }
    }
}

impl Instruction {
    fn new_direction(&self, dir: Compass) -> Compass {
        match self {
            Self::Left(_) => dir.counterclockwise().counterclockwise(),
            Self::Right(_) => dir.clockwise().clockwise(),
        }
    }
    fn distance(&self) -> i64 {
        match self {
            Self::Left(d) | Self::Right(d) => *d,
        }
    }
}

struct ChallengeInput {
    instructions: Vec<Instruction>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            instructions: value.split(", ").map(Instruction::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut dir = Compass::N;
        let mut pos = Point::default();
        for instruction in self.instructions.iter() {
            dir = instruction.new_direction(dir);
            let offset = Point::from(dir).scale(instruction.distance());
            pos = pos.add(offset);
        }
        pos.delta(Point::default())
    }
    fn solution_part_2(&self) -> i64 {
        let mut dir = Compass::N;
        let mut pos = Point::default();
        let mut seen: HashSet<Point> = HashSet::new();
        seen.insert(pos);
        'outer: for instruction in self.instructions.iter() {
            dir = instruction.new_direction(dir);

            for _ in 0..instruction.distance() {
                pos = pos.add(dir);
                if !seen.insert(pos) {
                    break 'outer;
                }
            }
        }
        pos.delta(Point::default())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, 242);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    assert_eq!(result_part2, 150);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_01_example_part_1.txt");

        let solutions = [5, 2, 12];

        for (input, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(input);

            let result_part1 = example.solution_part_1();
            println!("result day_01 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }

    #[test]
    fn test_example_day_01_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_01_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_01 part 2: {result_part2}");
        assert_eq!(result_part2, 4);

        Ok(())
    }
}
