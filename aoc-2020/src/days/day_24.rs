//!day_24.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point3D;
use std::collections::HashSet;

const E: Point3D = Point3D { x: 1, y: -1, z: 0 };
const W: Point3D = Point3D { x: -1, y: 1, z: 0 };
const NE: Point3D = Point3D { x: 1, y: 0, z: -1 };
const SW: Point3D = Point3D { x: -1, y: 0, z: 1 };
const NW: Point3D = Point3D { x: 0, y: 1, z: -1 };
const SE: Point3D = Point3D { x: 0, y: -1, z: 1 };

fn to_point3d(direction: &str) -> Point3D {
    match direction {
        "e" => E,
        "ne" => NE,
        "nw" => NW,
        "w" => W,
        "sw" => SW,
        "se" => SE,
        _ => panic!("unknown direction"),
    }
}

fn neighbors(hex: Point3D) -> impl Iterator<Item = Point3D> {
    [E, NE, NW, W, SW, SE].iter().map(move |dir| hex.add(dir))
}

struct ChallengeInput {
    instructions: String,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            instructions: value.to_string(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (usize, usize) {
        let mut black: HashSet<Point3D> = HashSet::new();
        for instructions in self.instructions.lines() {
            let target = self.parse_instruction_line_to_target(instructions);
            if !black.insert(target) {
                black.remove(&target);
            }
        }
        let result_part_1 = black.len();
        // part two
        for _ in 0..100 {
            black = self.check_black_and_white(black);
        }
        (result_part_1, black.len())
    }
    fn parse_instruction_line_to_target(&self, instructions: &str) -> Point3D {
        let mut slice_start = 0;
        let mut slice_end = 1;
        let mut position = Point3D::new(0, 0, 0);
        while slice_start < instructions.len() {
            let direction = match &instructions[slice_start..slice_end] {
                "e" | "ne" | "nw" | "w" | "sw" | "se" => {
                    to_point3d(&instructions[slice_start..slice_end])
                }
                "n" | "s" => {
                    slice_end += 1;
                    continue;
                }
                _ => panic!("unknown instruction char"),
            };
            position = position.add(&direction);
            slice_start = slice_end;
            slice_end += 1;
        }
        position
    }
    fn check_black_and_white(&self, black: HashSet<Point3D>) -> HashSet<Point3D> {
        let mut white: HashSet<Point3D> = HashSet::new();
        let mut new_black: HashSet<Point3D> = HashSet::new();
        for black_hex in black.iter() {
            let mut count_black = 0;
            for neighbor in neighbors(*black_hex) {
                if black.contains(&neighbor) {
                    count_black += 1;
                } else {
                    white.insert(neighbor);
                }
            }
            if count_black == 1 || count_black == 2 {
                new_black.insert(*black_hex);
            }
        }
        for white_hex in white {
            if neighbors(white_hex).filter(|w| black.contains(w)).count() == 2 {
                new_black.insert(white_hex);
            }
        }

        new_black
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_24.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_24 part 1: {result_part1}");
    assert_eq!(result_part1, 354);

    println!("result day_24 part 2: {result_part2}");
    assert_eq!(result_part2, 3_608);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_24() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_24_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_24 part 1: {result_part1}");
        assert_eq!(result_part1, 10);

        println!("result day_24 part 2: {result_part2}");
        assert_eq!(result_part2, 2_208);

        Ok(())
    }
}
