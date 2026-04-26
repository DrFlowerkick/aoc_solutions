//!day_11.rs

use anyhow::Result;
use my_lib::{
    my_geometry::my_point::Point3D,
    my_hex_grid::{FlatTopHex, HexCoordinates},
};

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { input: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1_and_2(&self) -> (i64, i64) {
        let mut current_pos = Point3D::default();
        let mut max_distance = i64::MIN;
        for step in self.input.split(",") {
            let dir = match step {
                "n" => Point3D::NORTH,
                "ne" => Point3D::EAST_NORTH,
                "se" => Point3D::EAST_SOUTH,
                "s" => Point3D::SOUTH,
                "sw" => Point3D::WEST_SOUTH,
                "nw" => Point3D::WEST_NORTH,
                _ => panic!("unknown direction"),
            };
            current_pos = current_pos.add(dir);
            max_distance = max_distance.max(current_pos.hex_distance(Point3D::default()));
        }
        (current_pos.hex_distance(Point3D::default()), max_distance)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, 761);

    println!("result day_11 part 2: {result_part2}");
    assert_eq!(result_part2, 1_542);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_11_example.txt");
        let solutions = [3, 0, 2, 3];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let (result_part1, _) = example.solution_part_1_and_2();
            println!("result day_11 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }
}
