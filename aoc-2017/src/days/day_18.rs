//!day_18.rs

use anyhow::Result;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
pub enum Value {
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
    pub fn value(&self, register: &Register) -> i64 {
        match self {
            Value::Char(reg) => *register.registers.get(reg).unwrap_or(&0),
            Value::Digit(digit) => *digit,
        }
    }
    pub fn register(&self) -> char {
        let Value::Char(c) = *self else {
            panic!("expected char")
        };
        c
    }
}

#[derive(Clone)]
pub struct Register {
    pub registers: HashMap<char, i64>,
    pub action_index: i64,
    pub actions: Vec<Action>,
    pub frequency: VecDeque<i64>,
    pub recovered_frequency: VecDeque<i64>,
    pub count_mul: u64,
}

impl Register {
    pub fn new(actions: Vec<Action>) -> Self {
        Self {
            registers: HashMap::new(),
            action_index: 0,
            actions,
            frequency: VecDeque::new(),
            recovered_frequency: VecDeque::new(),
            count_mul: 0,
        }
    }
    pub fn get_action(&self) -> Option<Action> {
        if self.action_index >= 0 && (self.action_index as usize) < self.actions.len() {
            self.actions.get(self.action_index as usize).copied()
        } else {
            None
        }
    }
    pub fn increment_index(&mut self) {
        self.action_index += 1;
    }
    pub fn set_frequency(&mut self, val: i64) {
        self.frequency.push_back(val);
    }
    pub fn set_recovered_frequency(&mut self, val: i64) {
        if val != 0
            && let Some(pop_val) = self.frequency.pop_back()
        {
            self.recovered_frequency.push_back(pop_val);
        }
    }
    pub fn jump_g(&mut self, val: i64, jump: i64) {
        if val > 0 {
            self.action_index += jump;
        } else {
            self.action_index += 1;
        }
    }
    pub fn jump_n(&mut self, val: i64, jump: i64) {
        if val != 0 {
            self.action_index += jump;
        } else {
            self.action_index += 1;
        }
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Snd(Value),
    Set(Value, Value),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Rcv(Value),
    Jgz(Value, Value),
    Jnz(Value, Value),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        let mut tokens = value.split_whitespace();
        match tokens.next().unwrap() {
            "snd" => Action::Snd(tokens.next().unwrap().into()),
            "set" => Action::Set(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "add" => Action::Add(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "sub" => Action::Sub(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "mul" => Action::Mul(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "mod" => Action::Mod(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "rcv" => Action::Rcv(tokens.next().unwrap().into()),
            "jgz" => Action::Jgz(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            "jnz" => Action::Jnz(tokens.next().unwrap().into(), tokens.next().unwrap().into()),
            _ => panic!("unknown token"),
        }
    }
}

impl Action {
    pub fn apply(&self, register: &mut Register) {
        match self {
            Action::Snd(reg) => {
                let reg = reg.value(register);
                register.set_frequency(reg);
                register.increment_index();
            }
            Action::Set(reg, val) => {
                let reg = reg.register();
                let val = val.value(register);
                register.registers.insert(reg, val);
                register.increment_index();
            }
            Action::Add(reg, val) => {
                let reg = reg.register();
                let val = val.value(register);
                register
                    .registers
                    .entry(reg)
                    .and_modify(|v| *v += val)
                    .or_insert(val);
                register.increment_index();
            }
            Action::Sub(reg, val) => {
                let reg = reg.register();
                let val = val.value(register);
                register
                    .registers
                    .entry(reg)
                    .and_modify(|v| *v -= val)
                    .or_insert(val);
                register.increment_index();
            }
            Action::Mul(reg, val) => {
                let reg = reg.register();
                let val = val.value(register);
                register.count_mul += 1;
                register.registers.entry(reg).and_modify(|v| *v *= val);
                register.increment_index();
            }
            Action::Mod(reg, val) => {
                let reg = reg.register();
                let val = val.value(register);
                register.registers.entry(reg).and_modify(|v| *v %= val);
                register.increment_index();
            }
            Action::Rcv(reg) => {
                let val = reg.value(register);
                register.set_recovered_frequency(val);
                register.increment_index();
            }
            Action::Jgz(reg, jump) => {
                let val = reg.value(register);
                let jump = jump.value(register);
                register.jump_g(val, jump);
            }
            Action::Jnz(reg, jump) => {
                let val = reg.value(register);
                let jump = jump.value(register);
                register.jump_n(val, jump);
            }
        }
    }
    pub fn extract_2nd_digit(&self) -> Option<i64> {
        let value = match self {
            Action::Add(_, v)
            | Action::Sub(_, v)
            | Action::Mod(_, v)
            | Action::Mul(_, v)
            | Action::Set(_, v) => v,
            _ => return None,
        };
        if let Value::Digit(value) = value {
            Some(*value)
        } else {
            None
        }
    }
}

pub struct ChallengeInput {
    pub actions: Vec<Action>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            actions: value.lines().map(Action::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_day_18(&self) -> i64 {
        let mut register = Register::new(self.actions.clone());
        while let Some(action) = register.get_action() {
            action.apply(&mut register);
            if let Some(frequency) = register.recovered_frequency.pop_front() {
                return frequency;
            }
        }
        0
    }
    fn solution_part_2_day_18(&self) -> usize {
        // Note: this solution results from re-engineering the register code sequences
        // my notes for this a private, because they contain may puzzle input, but
        // I think you can guess, what the register code sequence does by reading my rust code.
        let mut values = self.generate_values_to_sort();
        let mut counter = 0;
        let mut count = true;
        // pairwise sort until sorted
        loop {
            let mut sorted = true;
            for i in 0..values.len() - 1 {
                if values[i] < values[i + 1] {
                    values.swap(i, i + 1);
                    sorted = false
                }
            }

            // count only every second loop cycle (starting counting with first cycle)
            if count {
                counter += 1;
            }
            count = !count;

            // stop sort if sorted AND count is active
            if sorted && count {
                break;
            }
        }
        counter * values.len()
    }
    pub fn extract_digit_from_action(&self, index: usize) -> Option<i64> {
        self.actions.get(index).and_then(|a| a.extract_2nd_digit())
    }
    fn generate_values_to_sort(&self) -> Vec<i64> {
        let a: i64 = 2_i64.pow(31) - 1;
        // reading variables from puzzle input instead of hard coding them here
        let mut p = self.extract_digit_from_action(9).expect("unexpected value");
        let factor_1 = self
            .extract_digit_from_action(10)
            .expect("unexpected value");
        let factor_2 = self
            .extract_digit_from_action(12)
            .expect("unexpected value");
        let offset = self
            .extract_digit_from_action(13)
            .expect("unexpected value");
        let mod_quotient = self
            .extract_digit_from_action(16)
            .expect("unexpected value");
        let len = self.extract_digit_from_action(8).expect("unexpected value");
        (0..len)
            .map(|_| {
                p = ((p * factor_1).rem_euclid(a) * factor_2 + offset).rem_euclid(a);
                p.rem_euclid(mod_quotient)
            })
            .collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_18.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1_day_18();
    println!("result day_18 part 1: {result_part1}");
    assert_eq!(result_part1, 4_601);

    let result_part2 = challenge.solution_part_2_day_18();
    println!("result day_18 part 2: {result_part2}");
    assert_eq!(result_part2, 6_858);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_18_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1_day_18();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 4);

        Ok(())
    }

    #[test]
    fn text_value_generator() {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_18.txt");
        let challenge = ChallengeInput::from(input);

        let values = challenge.generate_values_to_sort();
        assert_eq!(values.last(), Some(&4_601));
    }
}
