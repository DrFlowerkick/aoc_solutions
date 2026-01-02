//!day_19.rs

use crate::days::day_16::Opcode;
use anyhow::Result;
use std::collections::HashSet;

struct Instruction {
    oc: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut input_iter = value.split_whitespace();
        Instruction {
            oc: Opcode::from(input_iter.next().unwrap()),
            a: input_iter.next().unwrap().parse().unwrap(),
            b: input_iter.next().unwrap().parse().unwrap(),
            c: input_iter.next().unwrap().parse().unwrap(),
        }
    }
}

impl Instruction {
    fn execute(&self, reg: Vec<usize>) -> Vec<usize> {
        self.oc.execute(self.a, self.b, self.c, reg)
    }
}

struct ChallengeInput {
    register_bound: usize,
    instructions: Vec<Instruction>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (register_bound, instructions) = value.split_once('\n').unwrap();
        let (_, register_bound) = register_bound.split_once(' ').unwrap();
        ChallengeInput {
            register_bound: register_bound.parse().unwrap(),
            instructions: instructions.lines().map(Instruction::from).collect(),
        }
    }
}

// the code does look for the sum of all divisors of a number, which will be calculated depending
// on initial register value of register 0. The number is written to register 1.
impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut instruction_pointer = 0;
        let mut register: Vec<usize> = vec![0; 6];
        while let Some(instruction) = self.instructions.get(instruction_pointer) {
            register[self.register_bound] = instruction_pointer;
            register = instruction.execute(register);
            instruction_pointer = register[self.register_bound] + 1;
        }
        // check with fast_solution
        assert_eq!(register[0], self.fast_solution(0));
        register[0]
    }
    fn solution_part_2(&self) -> usize {
        self.fast_solution(1)
    }
    fn fast_solution(&self, initial_value_of_register_0: usize) -> usize {
        let mut instruction_pointer = 0;
        let mut register: Vec<usize> = vec![0; 6];
        register[0] = initial_value_of_register_0;
        while let Some(instruction) = self.instructions.get(instruction_pointer) {
            register[self.register_bound] = instruction_pointer;
            register = instruction.execute(register);
            instruction_pointer = register[self.register_bound] + 1;
            if instruction_pointer == 1 {
                break;
            }
        }
        let dividend = register[1];
        let mut divisors: HashSet<usize> = HashSet::new();
        let upper_bound = ((dividend as f64).sqrt().round() as usize) + 1;
        for divisor in 1..=upper_bound {
            if dividend.is_multiple_of(divisor) {
                divisors.insert(divisor);
                divisors.insert(dividend / divisor);
            }
        }
        divisors.iter().sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_19.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, 2_821);

    let result_part2 = challenge.solution_part_2();
    println!("result day_19 part 2: {result_part2}");
    assert_eq!(result_part2, 30_529_296);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_19() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_19_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_19 part 1: {result_part1}");
        assert_eq!(result_part1, 6);

        // no example part 2

        Ok(())
    }
}
