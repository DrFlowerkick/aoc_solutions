//!day_12.rs

use anyhow::Result;
use my_lib::{
    my_compass::Compass,
    my_geometry::my_point::{Point, Turns90},
};

enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let val: i64 = value[1..].parse().unwrap();
        match &value[0..1] {
            "N" => Command::North(val),
            "S" => Command::South(val),
            "E" => Command::East(val),
            "W" => Command::West(val),
            "L" => Command::Left(val),
            "R" => Command::Right(val),
            "F" => Command::Forward(val),
            _ => panic!("unknown input"),
        }
    }
}

// transformation of Compass to Point: N is negative y, S is positive y
impl Command {
    fn apply(&self, pos: Point, mut orientation: Compass) -> (Point, Compass) {
        match self {
            Command::North(val) => {
                let n: Point = Compass::S.into();
                (pos.add(n.scale(*val)), orientation)
            }
            Command::South(val) => {
                let s: Point = Compass::N.into();
                (pos.add(s.scale(*val)), orientation)
            }
            Command::East(val) => {
                let e: Point = Compass::E.into();
                (pos.add(e.scale(*val)), orientation)
            }
            Command::West(val) => {
                let w: Point = Compass::W.into();
                (pos.add(w.scale(*val)), orientation)
            }
            Command::Forward(val) => {
                let inverted_orientation = match orientation {
                    Compass::N => Compass::S,
                    Compass::S => Compass::N,
                    comp => comp,
                };
                let f: Point = inverted_orientation.into();
                (pos.add(f.scale(*val)), orientation)
            }
            Command::Left(val) => {
                let num_turns = val / 45;
                for _ in 0..num_turns {
                    orientation = orientation.counterclockwise();
                }
                (pos, orientation)
            }
            Command::Right(val) => {
                let num_turns = val / 45;
                for _ in 0..num_turns {
                    orientation = orientation.clockwise();
                }
                (pos, orientation)
            }
        }
    }
    fn way_point(&self, pos: Point, way_point: Point) -> (Point, Point) {
        match self {
            Command::North(val) => {
                let n: Point = Compass::S.into();
                (pos, way_point.add(n.scale(*val)))
            }
            Command::South(val) => {
                let s: Point = Compass::N.into();
                (pos, way_point.add(s.scale(*val)))
            }
            Command::East(val) => {
                let e: Point = Compass::E.into();
                (pos, way_point.add(e.scale(*val)))
            }
            Command::West(val) => {
                let w: Point = Compass::W.into();
                (pos, way_point.add(w.scale(*val)))
            }
            Command::Forward(val) => {
                let f: Point = way_point.subtract(pos).scale(*val);
                (pos.add(f), way_point.add(f))
            }
            Command::Left(val) => {
                let turn = match val {
                    90 => Turns90::T90,
                    180 => Turns90::T180,
                    270 => Turns90::T270,
                    _ => unreachable!(),
                };
                (pos, way_point.turn_around_point(pos, turn, false))
            }
            Command::Right(val) => {
                let turn = match val {
                    90 => Turns90::T90,
                    180 => Turns90::T180,
                    270 => Turns90::T270,
                    _ => unreachable!(),
                };
                (pos, way_point.turn_around_point(pos, turn, true))
            }
        }
    }
}

struct ChallengeInput {
    commands: Vec<Command>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            commands: value.lines().map(Command::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut orientation = Compass::E;
        let mut pos = Point::new(0, 0);
        for command in self.commands.iter() {
            (pos, orientation) = command.apply(pos, orientation);
        }
        pos.x.abs() + pos.y.abs()
    }
    fn solution_part_2(&self) -> i64 {
        let mut pos = Point::new(0, 0);
        let mut way_point = Point::new(10, 1);
        for command in self.commands.iter() {
            (pos, way_point) = command.way_point(pos, way_point);
        }
        pos.x.abs() + pos.y.abs()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 796);

    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 39_446);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_12() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_12_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_12 part 1: {result_part1}");
        assert_eq!(result_part1, 25);

        let result_part2 = example.solution_part_2();
        println!("result day_12 part 2: {result_part2}");
        assert_eq!(result_part2, 286);

        Ok(())
    }
}
