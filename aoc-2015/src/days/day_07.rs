//!day_07.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Source<'a> {
    Signal(&'a str),
    Value(u16),
}

impl<'a> From<&'a str> for Source<'a> {
    fn from(value: &'a str) -> Self {
        if let Ok(val) = value.parse::<u16>() {
            Source::Value(val)
        } else {
            Source::Signal(value)
        }
    }
}

impl<'a> Source<'a> {
    fn get_value(&self, mapping: &HashMap<&str, Action>, seen: &mut HashMap<String, u16>) -> u16 {
        match self {
            Source::Signal(sig) => {
                let sig_string = sig.to_string();
                if let Some(val) = seen.get(&sig_string) {
                    return *val;
                }
                let action = mapping.get(sig).unwrap();
                let val = action.get_value(mapping, seen);
                seen.insert(sig_string, val);
                val
            }
            Source::Value(val) => *val,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action<'a> {
    Allocation(Source<'a>),
    And(Source<'a>, Source<'a>),
    Or(Source<'a>, Source<'a>),
    LShift(Source<'a>, u16),
    RShift(Source<'a>, u16),
    Not(Source<'a>),
}

impl<'a> From<&'a str> for Action<'a> {
    fn from(value: &'a str) -> Self {
        if let Some((left, right)) = value.split_once(" AND ") {
            Action::And(Source::from(left), Source::from(right))
        } else if let Some((left, right)) = value.split_once(" OR ") {
            Action::Or(Source::from(left), Source::from(right))
        } else if let Some((left, right)) = value.split_once(" LSHIFT ") {
            Action::LShift(Source::from(left), right.parse().unwrap())
        } else if let Some((left, right)) = value.split_once(" RSHIFT ") {
            Action::RShift(Source::from(left), right.parse().unwrap())
        } else if let Some(rem) = value.strip_prefix("NOT ") {
            Action::Not(Source::from(rem))
        } else {
            Action::Allocation(Source::from(value))
        }
    }
}

impl<'a> Action<'a> {
    fn get_value(&self, mapping: &HashMap<&str, Action>, seen: &mut HashMap<String, u16>) -> u16 {
        match self {
            Action::Allocation(src) => src.get_value(mapping, seen),
            Action::And(src_a, src_b) => {
                let a = src_a.get_value(mapping, seen);
                let b = src_b.get_value(mapping, seen);
                a & b
            }
            Action::Or(src_a, src_b) => {
                let a = src_a.get_value(mapping, seen);
                let b = src_b.get_value(mapping, seen);
                a | b
            }
            Action::LShift(src, shift) => src.get_value(mapping, seen) << shift,
            Action::RShift(src, shift) => src.get_value(mapping, seen) >> shift,
            Action::Not(src) => !src.get_value(mapping, seen),
        }
    }
}

struct ChallengeInput<'a> {
    mapping: HashMap<&'a str, Action<'a>>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput {
            mapping: value
                .lines()
                .map(|l| {
                    let (action, signal) = l.split_once(" -> ").unwrap();
                    (signal, Action::from(action))
                })
                .collect(),
        }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self, signal: &str) -> u16 {
        let mut seen: HashMap<String, u16> = HashMap::new();
        let action = self.mapping.get(signal).unwrap();
        action.get_value(&self.mapping, &mut seen)
    }
    fn solution_part_2(&mut self, a_part_1: u16) -> u16 {
        let b_action = Action::Allocation(Source::Value(a_part_1));
        self.mapping.insert("b", b_action);
        self.solution_part_1("a")
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_07.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1("a");
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, 3_176);

    let result_part2 = challenge.solution_part_2(result_part1);
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 14_710);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_07_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1("d");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 72);

        let result_part1 = example.solution_part_1("e");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 507);

        let result_part1 = example.solution_part_1("f");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 492);

        let result_part1 = example.solution_part_1("g");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 114);

        let result_part1 = example.solution_part_1("h");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 65_412);

        let result_part1 = example.solution_part_1("i");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 65_079);

        let result_part1 = example.solution_part_1("x");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 123);

        let result_part1 = example.solution_part_1("y");
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 456);

        Ok(())
    }
}
