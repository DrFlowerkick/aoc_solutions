//!day_08.rs

use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (instruction, value) = value.split_once(' ').unwrap();
        let value: i64 = value.parse().unwrap();
        match instruction {
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            "nop" => Instruction::Nop(value),
            _ => panic!("unknown instruction"),
        }
    }
}

impl Instruction {
    fn is_acc(&self) -> bool {
        matches!(self, Instruction::Acc(_))
    }
    fn switch(&self) -> Self {
        match self {
            Instruction::Acc(val) => Instruction::Acc(*val),
            Instruction::Jmp(val) => Instruction::Nop(*val),
            Instruction::Nop(val) => Instruction::Jmp(*val),
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
        match self.get_acc(-1) {
            Ok(_) => panic!("No loop occurred"),
            Err(acc_before_first_loop) => acc_before_first_loop,
        }
    }
    fn solution_part_2(&self) -> i64 {
        let non_acc_indices: Vec<_> = self
            .instructions
            .iter()
            .map(|i| !i.is_acc())
            .enumerate()
            .filter_map(|(i, f)| f.then_some(i as i64))
            .collect();
        for switch_index in non_acc_indices {
            if let Ok(val) = self.get_acc(switch_index) {
                return val;
            }
        }
        panic!("could not identify switch index")
    }
    fn get_acc(&self, switch_index: i64) -> Result<i64, i64> {
        let mut index = 0;
        let mut seen: HashSet<i64> = HashSet::new();
        let mut acc = 0;
        while seen.insert(index) {
            let Some(instruction) = self.instructions.get(index as usize) else {
                // index out of valid range
                if switch_index == -1 {
                    panic!("index out of range. should not happen in part 1.")
                }
                return Err(acc);
            };
            let instruction = if index == switch_index {
                instruction.switch()
            } else {
                *instruction
            };
            match instruction {
                Instruction::Acc(val) => {
                    acc += val;
                    index += 1;
                }
                Instruction::Jmp(val) => index += val,
                Instruction::Nop(_) => index += 1,
            }
            if (index as usize) == self.instructions.len() {
                // reached end of instruction list
                return Ok(acc);
            }
        }
        // loop detected, return acc before next loop
        Err(acc)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_08.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 1_782);

    let result_part2 = challenge.solution_part_2();
    println!("result day_08 part 2: {result_part2}");
    assert_eq!(result_part2, 797);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_08_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        let result_part2 = example.solution_part_2();
        println!("result day_08 part 2: {result_part2}");
        assert_eq!(result_part2, 8);

        Ok(())
    }
}
