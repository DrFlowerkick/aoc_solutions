//!day_21.rs

use crate::days::day_19::Instruction;
use anyhow::Result;
use std::collections::HashSet;

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

// If register[0] == 0, the code loops endless in a cycle of possible solutions.
// --> Collect all solutions and take the first for part 1 and the last for part 2.
// To reduce run time, we replace a slow running loop (starting at instruction 18
// in my puzzle input) with a direct calculation of the required value (see below).
impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (usize, usize) {
        let mut instruction_pointer = 0;
        let mut register: Vec<usize> = vec![0; 6];
        let mut first_solution = None::<usize>;
        let mut last_solution = 0;
        let mut solutions: HashSet<usize> = HashSet::new();
        while let Some(instruction) = self.instructions.get(instruction_pointer) {
            register[self.register_bound] = instruction_pointer;
            // pre processing for certain instructions (depends upon puzzle input)
            if instruction_pointer == 18 {
                // instead of incrementing register 5 one by one we directly set target value derived
                // from instructions "muli 2 256 2" and "gtrr 2 4 2"
                register[5] = register[4] / 256;
            } else if instruction_pointer == 28 {
                // collect possible solutions to halt program
                let solution = register[instruction.a];
                if !solutions.insert(solution) {
                    return (first_solution.unwrap(), last_solution);
                }
                if first_solution.is_none() {
                    first_solution = Some(solution);
                }
                last_solution = solution;
            }
            register = instruction.execute(register);
            instruction_pointer = register[self.register_bound] + 1;
        }
        unreachable!()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_21.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, 1_797_184);

    println!("result day_21 part 2: {result_part2}");
    assert_eq!(result_part2, 11_011_493);

    Ok(())
}

// No examples
