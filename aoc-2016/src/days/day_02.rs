//!day_02.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashMap;

fn get_keypad_part_1() -> HashMap<Point, char> {
    let mut keypad = HashMap::new();
    keypad.insert(Point::new(0, 0), '1');
    keypad.insert(Point::new(1, 0), '2');
    keypad.insert(Point::new(2, 0), '3');
    keypad.insert(Point::new(0, 1), '4');
    keypad.insert(Point::new(1, 1), '5');
    keypad.insert(Point::new(2, 1), '6');
    keypad.insert(Point::new(0, 2), '7');
    keypad.insert(Point::new(1, 2), '8');
    keypad.insert(Point::new(2, 2), '9');
    keypad
}

fn get_keypad_part_2() -> HashMap<Point, char> {
    let mut keypad = HashMap::new();
    keypad.insert(Point::new(2, 0), '1');
    keypad.insert(Point::new(1, 1), '2');
    keypad.insert(Point::new(2, 1), '3');
    keypad.insert(Point::new(3, 1), '4');
    keypad.insert(Point::new(0, 2), '5');
    keypad.insert(Point::new(1, 2), '6');
    keypad.insert(Point::new(2, 2), '7');
    keypad.insert(Point::new(3, 2), '8');
    keypad.insert(Point::new(4, 2), '9');
    keypad.insert(Point::new(1, 3), 'A');
    keypad.insert(Point::new(2, 3), 'B');
    keypad.insert(Point::new(3, 3), 'C');
    keypad.insert(Point::new(2, 4), 'D');
    keypad
}

struct ChallengeInput {
    lines_of_instructions: Vec<Vec<Compass>>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            lines_of_instructions: value
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            'U' => Compass::N,
                            'R' => Compass::E,
                            'D' => Compass::S,
                            'L' => Compass::W,
                            _ => panic!("unknown direction"),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> String {
        self.use_keypad(Point::new(1, 1), get_keypad_part_1())
    }
    fn solution_part_2(&self) -> String {
        self.use_keypad(Point::new(0, 2), get_keypad_part_2())
    }
    fn use_keypad(&self, mut pos: Point, keypad: HashMap<Point, char>) -> String {
        let mut code = String::new();
        for line in self.lines_of_instructions.iter() {
            for direction in line.iter() {
                let new_pos = pos.add(*direction);
                if keypad.contains_key(&new_pos) {
                    pos = new_pos;
                }
            }
            code.push(*keypad.get(&pos).unwrap());
        }
        code
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, "92435");

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, "C1A88");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_02_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, "1985");

        let result_part2 = example.solution_part_2();
        println!("result day_02 part 2: {result_part2}");
        assert_eq!(result_part2, "5DB3");

        Ok(())
    }
}
