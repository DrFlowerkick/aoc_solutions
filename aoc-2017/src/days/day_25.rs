//!day_25.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Action {
    value: bool,
    i_move: i64,
    state: char,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        let mut lines = value.trim().lines();
        let value = lines.next().unwrap().trim() == "- Write the value 1.";
        let i_move = if lines.next().unwrap().trim() == "- Move one slot to the right." {
            1
        } else {
            -1
        };
        let state = lines.next().unwrap().trim().chars().nth(22).unwrap();
        Self {
            value,
            i_move,
            state,
        }
    }
}

struct ChallengeInput {
    initial_state: char,
    steps: u64,
    machine: HashMap<(char, bool), Action>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut states = value.split("\n\n");
        let init = states.next().unwrap();
        let initial_state = init.chars().nth(15).unwrap();
        let steps = init
            .split_whitespace()
            .filter_map(|d| d.parse().ok())
            .next()
            .unwrap();
        let machine = states
            .flat_map(|s| {
                let mut state_iter = s.split("If the current value is ");
                let name = state_iter.next().unwrap().trim().chars().nth(9).unwrap();
                let (value, action) = state_iter.next().unwrap().split_once(":").unwrap();
                let value_a = value == "1";
                let action_a = Action::from(action.trim());
                let (value, action) = state_iter.next().unwrap().split_once(":").unwrap();
                let value_b = value == "1";
                let action_b = Action::from(action.trim());
                [((name, value_a), action_a), ((name, value_b), action_b)].into_iter()
            })
            .collect();
        ChallengeInput {
            initial_state,
            steps,
            machine,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut tape: HashMap<i64, bool> = HashMap::new();
        let mut index = 0;
        let mut state = self.initial_state;
        for _ in 0..self.steps {
            let value = tape.entry(index).or_insert(false);
            let action = self.machine.get(&(state, *value)).unwrap();
            *value = action.value;
            index += action.i_move;
            state = action.state;
        }
        tape.values().filter(|v| **v).count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_25.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_25 part 1: {result_part1}");
    assert_eq!(result_part1, 4_387);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_25() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_25_example.txt");
        let example = ChallengeInput::from(input);
        assert_eq!(example.initial_state, 'A');
        assert_eq!(example.steps, 6);

        let result_part1 = example.solution_part_1();
        println!("result day_25 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        Ok(())
    }
}
