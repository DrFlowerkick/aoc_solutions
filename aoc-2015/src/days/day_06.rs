//!day_06.rs

use anyhow::Result;
use my_lib::my_geometry::{my_point::Point, my_rectangle::Rectangle};
use std::collections::VecDeque;

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
    fn apply_to_off(&self) -> Option<Rectangle> {
        match self.action {
            Action::On | Action::Toggle => Some(self.area),
            Action::Off => None,
        }
    }
    fn new_remaining(&self, remaining_area: Rectangle) -> Self {
        Instruction {
            action: self.action,
            area: remaining_area,
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
    fn solution_part_1(&self) -> i64 {
        let mut on: Vec<Rectangle> = Vec::new();
        for instruction in self.instructions.iter() {
            let mut queue: VecDeque<Instruction> = VecDeque::new();
            queue.push_back(*instruction);
            while let Some(ins) = queue.pop_front() {
                if let Some((intersection, index)) = on
                    .iter()
                    .enumerate()
                    .find_map(|(i, o)| o.rectangle_intersection(&ins.area, true).map(|r| (r, i)))
                {
                    // apply action to intersection, collect remaining area and than continue with while loop

                    // if action is on, keep on as is
                    // else remove intersecting area from on, cut intersection and keep remaining areas as on
                    if ins.action != Action::On {
                        // pop intersecting area from on
                        let old_on = on.swap_remove(index);
                        // get remaining areas of old_on and add them to on
                        for rem_on in old_on.cut_rectangle(&intersection, true) {
                            on.push(rem_on);
                        }
                    }

                    // cut remaining areas from instruction and add them to queue
                    for rem_ins in ins.area.cut_rectangle(&intersection, true) {
                        queue.push_back(ins.new_remaining(rem_ins));
                    }

                    continue;
                }
                // found no intersection: apply instruction to off area
                if let Some(new_on) = ins.apply_to_off() {
                    on.push(new_on);
                }
            }
        }
        on.iter().map(|r| r.surface_inclusive()).sum()
    }
    fn solution_part_2(&self) -> u64 {
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_06.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_06 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_06 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

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
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
