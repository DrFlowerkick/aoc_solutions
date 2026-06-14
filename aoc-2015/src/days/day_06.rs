//!day_06.rs

use anyhow::Result;
use my_lib::my_geometry::{my_point::Point, my_rectangle::Rectangle};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "on" => Action::On,
            "off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => panic!("unknown action"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    action: Action,
    area: Rectangle,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let value = if let Some(stripped) = value.strip_prefix("turn ") {
            stripped
        } else {
            value
        };
        let (action, area) = value.split_once(" ").unwrap();
        let action = Action::from(action);
        let (point_a, point_b) = area.split_once(" through ").unwrap();
        let point_a = Point::try_from(point_a).unwrap();
        let point_b = Point::try_from(point_b).unwrap();
        let area = Rectangle::from((point_a, point_b));
        Instruction { action, area }
    }
}

impl Instruction {
    fn apply(&self, on: &mut HashMap<Point, bool>) {
        for y in self.area.corners()[3].y..=self.area.corners()[0].y {
            for x in self.area.corners()[0].x..=self.area.corners()[3].x {
                match self.action {
                    Action::On => {
                        on.entry((x, y).into())
                            .and_modify(|v| *v = true)
                            .or_insert(true);
                    }
                    Action::Off => {
                        on.entry((x, y).into())
                            .and_modify(|v| *v = false)
                            .or_insert(false);
                    }
                    Action::Toggle => {
                        on.entry((x, y).into())
                            .and_modify(|v| *v = !*v)
                            .or_insert(true);
                    }
                }
            }
        }
    }
    fn apply_part2(&self, on: &mut HashMap<Point, u64>) {
        for y in self.area.corners()[3].y..=self.area.corners()[0].y {
            for x in self.area.corners()[0].x..=self.area.corners()[3].x {
                match self.action {
                    Action::On => {
                        on.entry((x, y).into()).and_modify(|v| *v += 1).or_insert(1);
                    }
                    Action::Off => {
                        on.entry((x, y).into())
                            .and_modify(|v| *v = v.saturating_sub(1))
                            .or_insert(0);
                    }
                    Action::Toggle => {
                        on.entry((x, y).into()).and_modify(|v| *v += 2).or_insert(2);
                    }
                }
            }
        }
    }
}

struct ChallengeInput {
    instructions: Vec<Instruction>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            instructions: value.lines().map(Instruction::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut on: HashMap<Point, bool> = HashMap::with_capacity(1000 * 1000);
        for instruction in self.instructions.iter() {
            instruction.apply(&mut on);
        }
        on.values().filter(|r| **r).count()
    }
    fn solution_part_2(&self) -> u64 {
        let mut on: HashMap<Point, u64> = HashMap::with_capacity(1000 * 1000);
        for instruction in self.instructions.iter() {
            instruction.apply_part2(&mut on);
        }
        on.values().copied().sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_06.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 377_891);

    let result_part2 = challenge.solution_part_2();
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 14_110_788);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_06_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 1000 * 1000 - 1000 - 4);

        let result_part2 = example.solution_part_2();
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 1000 * 1000 + 2000 - 4);

        Ok(())
    }
}
