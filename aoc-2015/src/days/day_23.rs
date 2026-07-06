//!day_23.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i64),
    Jie(char, i64),
    Jio(char, i64),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (ins, rem) = value.split_once(" ").unwrap();
        match ins {
            "hlf" => Instruction::Hlf(rem.parse().unwrap()),
            "tpl" => Instruction::Tpl(rem.parse().unwrap()),
            "inc" => Instruction::Inc(rem.parse().unwrap()),
            "jmp" => Instruction::Jmp(rem.parse().unwrap()),
            "jie" => {
                let (reg, offset) = rem.split_once(", ").unwrap();
                Instruction::Jie(reg.parse().unwrap(), offset.parse().unwrap())
            }
            "jio" => {
                let (reg, offset) = rem.split_once(", ").unwrap();
                Instruction::Jio(reg.parse().unwrap(), offset.parse().unwrap())
            }
            _ => panic!("unknown instruction"),
        }
    }
}

impl Instruction {
    // returns offset to index of instructions
    fn apply_to_registers(&self, registers: &mut HashMap<char, u64>) -> i64 {
        match self {
            Instruction::Hlf(r) => {
                registers.entry(*r).and_modify(|v| *v /= 2).or_insert(0);
                1
            }
            Instruction::Tpl(r) => {
                registers.entry(*r).and_modify(|v| *v *= 3).or_insert(0);
                1
            }
            Instruction::Inc(r) => {
                registers.entry(*r).and_modify(|v| *v += 1).or_insert(1);
                1
            }
            Instruction::Jmp(o) => *o,
            Instruction::Jie(r, o) => {
                if registers.get(r).copied().unwrap_or_default() & 1 == 0 {
                    *o
                } else {
                    1
                }
            }
            Instruction::Jio(r, o) => {
                if registers.get(r).copied().unwrap_or_default() == 1 {
                    *o
                } else {
                    1
                }
            }
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
    fn solution_part_1(&self, reg: char) -> u64 {
        let mut registers: HashMap<char, u64> = HashMap::new();
        self.execute_instructions(&mut registers);
        registers.get(&reg).copied().unwrap_or_default()
    }
    fn solution_part_2(&self) -> u64 {
        let mut registers: HashMap<char, u64> = HashMap::new();
        registers.insert('a', 1);
        self.execute_instructions(&mut registers);
        registers.get(&'b').copied().unwrap_or_default()
    }
    fn execute_instructions(&self, registers: &mut HashMap<char, u64>) {
        let mut index = 0_i64;
        while index >= 0
            && let Some(ins) = self.instructions.get(index as usize)
        {
            index += ins.apply_to_registers(registers);
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_23.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1('b');
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, 307);

    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 160);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_23_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1('a');
        println!("result day_23 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
