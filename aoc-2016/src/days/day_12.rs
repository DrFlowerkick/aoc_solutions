//!day_12.rs

use anyhow::Result;
use std::collections::HashMap;

type Register = HashMap<char, i64>;

#[derive(Clone, Copy)]
enum Value {
    Digit(i64),
    Char(char),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let c = value.chars().next().unwrap();
        if c.is_alphabetic() {
            Value::Char(c)
        } else {
            Value::Digit(value.parse().unwrap())
        }
    }
}

impl Value {
    fn value(&self, register: &Register) -> i64 {
        match self {
            Value::Char(reg) => *register.get(reg).unwrap_or(&0),
            Value::Digit(digit) => *digit,
        }
    }
    fn register(&self) -> char {
        let Value::Char(c) = *self else {
            panic!("expected char")
        };
        c
    }
}

#[derive(Clone, Copy)]
enum Action {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        let mut tokens = value.split_whitespace();
        match tokens.next().unwrap() {
            "cpy" => Action::Cpy(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "inc" => Action::Inc(tokens.next().unwrap().into()),
            "dec" => Action::Dec(tokens.next().unwrap().into()),
            "jnz" => Action::Jnz(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            _ => panic!("unknown token"),
        }
    }
}

impl Action {
    fn apply(&self, register: &mut Register) -> i64 {
        match self {
            Action::Cpy(val, reg) => {
                let reg = reg.register();
                let val = val.value(register);
                register.insert(reg, val);
                1
            }
            Action::Inc(reg) => {
                let reg = reg.register();
                register.entry(reg).and_modify(|v| *v += 1).or_insert(1);
                1
            }
            Action::Dec(reg) => {
                let reg = reg.register();
                register.entry(reg).and_modify(|v| *v -= 1).or_insert(-1);
                1
            }
            Action::Jnz(reg, jump) => {
                let val = reg.value(register);
                let jump = jump.value(register);
                if val != 0 { jump } else { 1 }
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
            actions: value.lines().map(Action::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut index = 0;
        let num_actions = self.actions.len() as i64;
        let mut register: Register = HashMap::new();
        while index >= 0 && index < num_actions {
            index += self.actions[index as usize].apply(&mut register);
        }
        *register.get(&'a').unwrap()
    }
    fn solution_part_2(&self) -> i64 {
        let mut index = 0;
        let num_actions = self.actions.len() as i64;
        let mut register: Register = HashMap::new();
        register.insert('c', 1);
        while index >= 0 && index < num_actions {
            index += self.actions[index as usize].apply(&mut register);
        }
        *register.get(&'a').unwrap()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 317_993);

    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 9_227_647);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_12() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_12_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_12 part 1: {result_part1}");
        assert_eq!(result_part1, 42);

        Ok(())
    }
}
