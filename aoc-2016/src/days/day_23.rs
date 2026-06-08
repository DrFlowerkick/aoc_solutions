//!day_23.rs

use super::day_12::{Action, Register};
use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    actions: Vec<Action>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            actions: value.lines().map(Action::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut register: Register = HashMap::new();
        register.insert('a', 7);
        let mut index = 0;
        let mut actions = self.actions.clone();
        let num_actions = self.actions.len() as i64;
        while index >= 0 && index < num_actions {
            let (delta_index, toggle) = actions[index as usize].apply(&mut register);
            if let Some(delta_toggle) = toggle {
                let toggle_index = index + delta_toggle;
                if toggle_index >= 0 && toggle_index < num_actions {
                    actions[toggle_index as usize] = actions[toggle_index as usize].toggle();
                }
            }
            index += delta_index;
        }
        *register.get(&'a').unwrap()
    }
    fn solution_part_2(&self) -> i64 {
        let mut register: Register = HashMap::new();
        register.insert('a', 12);
        let mut index = 0;
        let mut actions = self.actions.clone();
        let num_actions = self.actions.len() as i64;
        while index >= 0 && index < num_actions {
            if index == 8 {
                let d = *register.get(&'d').unwrap();
                register.entry('a').and_modify(|a| *a *= d);
                register.entry('d').and_modify(|d| *d = 0);
                index += 1;
            } else {
                let (delta_index, toggle) = actions[index as usize].apply(&mut register);
                if let Some(delta_toggle) = toggle {
                    let toggle_index = index + delta_toggle;
                    if toggle_index >= 0 && toggle_index < num_actions {
                        actions[toggle_index as usize] = actions[toggle_index as usize].toggle();
                    }
                }
                index += delta_index;
            }
        }
        *register.get(&'a').unwrap()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_23.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, 10_223);

    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 479_006_783);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_23_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        Ok(())
    }
}
