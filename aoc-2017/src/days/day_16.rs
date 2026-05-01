//!day_16.rs

use anyhow::Result;
use std::collections::HashMap;

fn generate_chars(size: u8) -> Vec<char> {
    (0..size).map(|o| (97 + o) as char).collect()
}

#[derive(Clone, Copy)]
enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match &value[..1] {
            "s" => Action::Spin(value[1..].parse().unwrap()),
            "x" => {
                let (a, b) = value[1..].split_once("/").unwrap();
                Action::Exchange(a.parse().unwrap(), b.parse().unwrap())
            }
            "p" => {
                let (a, b) = value[1..].split_once("/").unwrap();
                Action::Partner(a.chars().next().unwrap(), b.chars().next().unwrap())
            }
            _ => panic!("unknown action char"),
        }
    }
}

impl Action {
    fn apply(&self, slice: &mut [char]) {
        match *self {
            Action::Spin(rot) => slice.rotate_right(rot),
            Action::Exchange(a, b) => slice.swap(a, b),
            Action::Partner(a, b) => {
                let pos_a = slice.iter().position(|c| *c == a).unwrap();
                let pos_b = slice.iter().position(|c| *c == b).unwrap();
                slice.swap(pos_a, pos_b);
            }
        }
    }
}

struct ChallengeInput {
    actions: Vec<Action>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            actions: value.split(",").map(Action::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self, chars: &mut [char]) -> (String, String) {
        let mut seen: HashMap<String, usize> = HashMap::new();
        let mut cycle = 0;
        for action in self.actions.iter() {
            action.apply(chars);
        }
        let part_1 = chars.iter().collect();

        // exit here if test
        if chars.len() == 5 {
            return (part_1, "".into());
        }

        // part 2
        cycle += 1;
        seen.insert(part_1.clone(), cycle);
        let mut current: String;

        // find first instance of cycle pattern
        loop {
            for action in self.actions.iter() {
                action.apply(chars);
            }
            cycle += 1;
            current = chars.iter().collect();
            if seen.insert(current.clone(), cycle).is_some() {
                break;
            }
        }
        let first_cycle = cycle;
        let cycle_pattern = current.clone();

        // find second instance of cycle pattern
        loop {
            for action in self.actions.iter() {
                action.apply(chars);
            }
            cycle += 1;
            current = chars.iter().collect();
            if current == cycle_pattern {
                break;
            }
        }
        let second_cycle = cycle;

        // check for constant cycle
        loop {
            for action in self.actions.iter() {
                action.apply(chars);
            }
            cycle += 1;
            current = chars.iter().collect();
            if current == cycle_pattern {
                break;
            }
        }
        assert_eq!(second_cycle - first_cycle, cycle - second_cycle);

        // calc remaining cycles and find final pattern
        let remaining_cycles =
            (1_000_000_000_usize - first_cycle).rem_euclid(second_cycle - first_cycle);
        for _ in 0..remaining_cycles {
            for action in self.actions.iter() {
                action.apply(chars);
            }
        }

        (part_1, chars.iter().collect())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_16.txt");
    let challenge = ChallengeInput::from(input);

    let mut chars = generate_chars(16);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(&mut chars);
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, "ebjpfdgmihonackl");

    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, "abocefghijklmndp");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_16() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_16_example.txt");
        let example = ChallengeInput::from(input);

        let mut chars = generate_chars(5);

        let (result_part1, _) = example.solution_part_1_and_2(&mut chars);
        println!("result day_16 part 1: {result_part1}");
        assert_eq!(result_part1, "baedc");

        Ok(())
    }
}
