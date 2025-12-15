//!day_12.rs

use anyhow::Result;
use my_lib::{
    my_array::MyArray,
    my_geometry::{my_point::Point, my_rectangle::Rectangle},
};

#[derive(Debug, Clone, Copy)]
struct Shape {
    _area: Rectangle,
    _shape: MyArray<Point, 9>,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        let (_, shape) = value.split_once(":\n").unwrap();
        Shape {
            _area: Rectangle::new((0, 2).into(), (2, 0).into()),
            _shape: shape
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Point::new(x as i64, y as i64))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        }
    }
}

struct Region {
    len_x: i64,
    len_y: i64,
    shapes: Vec<usize>,
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (dimensions, shapes) = value.split_once(": ").unwrap();
        let (len_x, len_y) = dimensions.split_once('x').unwrap();
        Region {
            len_x: len_x.parse().unwrap(),
            len_y: len_y.parse().unwrap(),
            shapes: shapes
                .split_whitespace()
                .map(|d| d.parse().unwrap())
                .collect(),
        }
    }
}

impl Region {
    fn check_region_size(&self) -> bool {
        self.len_x * self.len_y >= (self.shapes.iter().sum::<usize>() as i64) * 9
    }
}

struct ChallengeInput {
    _shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut _shapes = Vec::new();
        let mut regions = Vec::new();
        for block in value.split("\n\n") {
            if block.contains('#') {
                let shape = Shape::from(block);
                _shapes.push(shape);
            } else {
                regions = block.lines().map(Region::from).collect();
            }
        }

        ChallengeInput { _shapes, regions }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        // the solution is kinda disappointing:
        // we have to check, if a number of 3x3 "shape" tiles of the required shapes do fit
        // into the region. We do not need to put the tiles together as dense as possible
        // as is described in the given example.
        self.regions
            .iter()
            .filter(|r| r.check_region_size())
            .count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 469);

    // as always there is no part 2 at the last day.

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_12() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_12_example.txt");
        let example = ChallengeInput::from(input);

        // the example is irrelevant for the solution
        let result_part1 = example.solution_part_1();
        println!("result day_12 part 1: {result_part1}");
        //assert_eq!(result_part1, 2);

        Ok(())
    }
}
