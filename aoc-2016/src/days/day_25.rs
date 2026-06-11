//!day_25.rs

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
        let mut actions = self.actions.clone();
        let num_actions = self.actions.len() as i64;
        'outer: for a in 0..i64::MAX {
            let mut last_out = None::<i64>;
            let mut out_count = 0;
            let mut register: Register = HashMap::new();
            register.insert('a', a);
            let mut index = 0;
            while index >= 0 && index < num_actions {
                let (delta_index, toggle, out) = actions[index as usize].apply(&mut register);
                if let Some(delta_toggle) = toggle {
                    let toggle_index = index + delta_toggle;
                    if toggle_index >= 0 && toggle_index < num_actions {
                        actions[toggle_index as usize] = actions[toggle_index as usize].toggle();
                    }
                }
                if let Some(next_out) = out {
                    if next_out > 1 {
                        continue 'outer;
                    }
                    if let Some(lo) = last_out
                        && lo == next_out
                    {
                        continue 'outer;
                    }
                    out_count += 1;
                    if out_count == 200 {
                        return a;
                    }
                    last_out = out;
                }
                index += delta_index;
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_25.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_25 part 1: {result_part1}");
    assert_eq!(result_part1, 158);

    Ok(())
}

// No tests for day 25
