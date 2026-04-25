//!day_08.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Action {
    Increment,
    Decrement,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "inc" => Action::Increment,
            "dec" => Action::Decrement,
            _ => panic!("unknown action str"),
        }
    }
}

impl Action {
    fn apply(&self, register: i64, modifier: i64) -> i64 {
        match self {
            Action::Increment => register + modifier,
            Action::Decrement => register - modifier,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Comparator {
    Greater,
    GreaterEqual,
    Equal,
    SmallerEqual,
    Smaller,
    NotEqual,
}

impl From<&str> for Comparator {
    fn from(value: &str) -> Self {
        match value {
            ">" => Comparator::Greater,
            ">=" => Comparator::GreaterEqual,
            "==" => Comparator::Equal,
            "<=" => Comparator::SmallerEqual,
            "<" => Comparator::Smaller,
            "!=" => Comparator::NotEqual,
            _ => panic!("unknown comparator str"),
        }
    }
}

impl Comparator {
    fn compare(&self, register: i64, conditional: i64) -> bool {
        match self {
            Comparator::Greater => register > conditional,
            Comparator::GreaterEqual => register >= conditional,
            Comparator::Equal => register == conditional,
            Comparator::SmallerEqual => register <= conditional,
            Comparator::Smaller => register < conditional,
            Comparator::NotEqual => register != conditional,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    register: &'a str,
    action: Action,
    modifier: i64,
    condition_register: &'a str,
    comparator: Comparator,
    conditional: i64,
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        let mut iter = value.split_whitespace();
        let register = iter.next().unwrap();
        let action = Action::from(iter.next().unwrap());
        let modifier = iter.next().unwrap().parse().unwrap();
        iter.next();
        let condition_register = iter.next().unwrap();
        let comparator = Comparator::from(iter.next().unwrap());
        let conditional = iter.next().unwrap().parse().unwrap();
        Instruction {
            register,
            action,
            modifier,
            condition_register,
            comparator,
            conditional,
        }
    }
}

impl<'a> Instruction<'a> {
    fn apply(&'a self, registers: &mut HashMap<&'a str, i64>) {
        let register_value = registers.get(self.register).unwrap_or(&0);
        let condition_register_value = registers.get(self.condition_register).unwrap_or(&0);
        if self
            .comparator
            .compare(*condition_register_value, self.conditional)
        {
            let new_value = self.action.apply(*register_value, self.modifier);
            registers.insert(self.register, new_value);
        }
    }
}

struct ChallengeInput<'a> {
    instructions: Vec<Instruction<'a>>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput {
            instructions: value.lines().map(Instruction::from).collect(),
        }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1_and_2(&self) -> (i64, i64) {
        let mut registers = HashMap::new();
        let mut max = i64::MIN;
        for instruction in self.instructions.iter() {
            instruction.apply(&mut registers);
            if let Some(max_register) = registers.values().max() {
                max = max.max(*max_register);
            }
        }
        (*registers.values().max().unwrap(), max)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_08.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 4_832);

    println!("result day_08 part 2: {result_part2}");
    assert_eq!(result_part2, 5_443);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_08_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 1);

        println!("result day_08 part 2: {result_part2}");
        assert_eq!(result_part2, 10);

        Ok(())
    }
}
