//!day_16.rs

use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Register = [usize; 4];
type Instruction = (usize, usize, usize, usize);

#[derive(Debug, Clone, Copy)]
struct Sample {
    before: Register,
    instruction: Instruction,
    after: Register,
}

impl From<&str> for Sample {
    fn from(value: &str) -> Self {
        let re = Regex::new(
            r"Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]",
        )
        .unwrap();
        let caps: Vec<usize> = re
            .captures(value)
            .unwrap()
            .iter()
            .skip(1)
            .flatten()
            .map(|c| c.as_str().parse().unwrap())
            .collect();
        Sample {
            before: [caps[0], caps[1], caps[2], caps[3]],
            instruction: (caps[4], caps[5], caps[6], caps[7]),
            after: [caps[8], caps[9], caps[10], caps[11]],
        }
    }
}

impl Sample {
    fn try_opcode(&self, oc: &Opcode) -> bool {
        oc.execute(
            self.instruction.1,
            self.instruction.2,
            self.instruction.3,
            self.before,
        ) == self.after
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const ALL_OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

impl Opcode {
    fn execute(&self, a: usize, b: usize, c: usize, mut reg: Register) -> Register {
        match self {
            Opcode::Addr => {
                reg[c] = reg[a] + reg[b];
                reg
            }
            Opcode::Addi => {
                reg[c] = reg[a] + b;
                reg
            }
            Opcode::Mulr => {
                reg[c] = reg[a] * reg[b];
                reg
            }
            Opcode::Muli => {
                reg[c] = reg[a] * b;
                reg
            }
            Opcode::Banr => {
                reg[c] = reg[a] & reg[b];
                reg
            }
            Opcode::Bani => {
                reg[c] = reg[a] & b;
                reg
            }
            Opcode::Borr => {
                reg[c] = reg[a] | reg[b];
                reg
            }
            Opcode::Bori => {
                reg[c] = reg[a] | b;
                reg
            }
            Opcode::Setr => {
                reg[c] = reg[a];
                reg
            }
            Opcode::Seti => {
                reg[c] = a;
                reg
            }
            Opcode::Gtir => {
                reg[c] = if a > reg[b] { 1 } else { 0 };
                reg
            }
            Opcode::Gtri => {
                reg[c] = if reg[a] > b { 1 } else { 0 };
                reg
            }
            Opcode::Gtrr => {
                reg[c] = if reg[a] > reg[b] { 1 } else { 0 };
                reg
            }
            Opcode::Eqir => {
                reg[c] = if a == reg[b] { 1 } else { 0 };
                reg
            }
            Opcode::Eqri => {
                reg[c] = if reg[a] == b { 1 } else { 0 };
                reg
            }
            Opcode::Eqrr => {
                reg[c] = if reg[a] == reg[b] { 1 } else { 0 };
                reg
            }
        }
    }
}

struct ChallengeInput {
    samples: Vec<Sample>,
    instructions: Vec<Instruction>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (samples, instructions) = value.split_once("\n\n\n\n").unwrap();
        let re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
        ChallengeInput {
            samples: samples.split("\n\n").map(Sample::from).collect(),
            instructions: instructions
                .lines()
                .map(|l| {
                    let caps = re.captures(l).unwrap();
                    (
                        caps[1].parse().unwrap(),
                        caps[2].parse().unwrap(),
                        caps[3].parse().unwrap(),
                        caps[4].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.samples
            .iter()
            .filter(|s| {
                ALL_OPCODES
                    .into_iter()
                    .filter(|oc| s.try_opcode(oc))
                    .count()
                    >= 3
            })
            .count()
    }
    fn solution_part_2(&self) -> usize {
        let id_map = self.identify_opcode_id();
        let mut register = [0, 0, 0, 0];
        for (op_id, a, b, c) in self.instructions.iter() {
            let oc_index = id_map.get(op_id).unwrap();
            let oc = ALL_OPCODES[*oc_index];
            register = oc.execute(*a, *b, *c, register);
        }
        register[0]
    }
    fn identify_opcode_id(&self) -> HashMap<usize, usize> {
        // Map: Opcode ID (Input) -> Possible Enum Indices (0..15)
        let mut opcode_to_indices: HashMap<usize, HashSet<usize>> = HashMap::new();
        // Map: Enum Index (0..15) -> Possible Opcode IDs (Input)
        let mut index_to_opcodes: HashMap<usize, HashSet<usize>> = HashMap::new();

        // analyze samples
        for sample in self.samples.iter() {
            let matches: HashSet<usize> = ALL_OPCODES
                .into_iter()
                .enumerate()
                .filter(|(_, oc)| sample.try_opcode(oc))
                .map(|(i, _)| i)
                .collect();

            opcode_to_indices
                .entry(sample.instruction.0)
                .and_modify(|set| {
                    // only keep indices, which are in both hash sets
                    *set = set.intersection(&matches).copied().collect();
                })
                .or_insert(matches);
        }

        // "reverse" mapping of index to opcode
        for i in 0..16 {
            index_to_opcodes.insert(i, HashSet::new());
            for (op_id, indices) in &opcode_to_indices {
                if indices.contains(&i) {
                    index_to_opcodes.entry(i).and_modify(|set| {
                        set.insert(*op_id);
                    });
                }
            }
        }

        let mut id_map: HashMap<usize, usize> = HashMap::new();
        while id_map.len() < 16 {
            let mut found = None;
            if let Some((op_id, single_index)) =
                opcode_to_indices.iter().find(|(_, set)| set.len() == 1)
            {
                // found set with single index
                let single_index = *single_index.iter().next().unwrap();
                found = Some((*op_id, single_index));
            } else if let Some((index, single_op_id)) =
                index_to_opcodes.iter().find(|(_, set)| set.len() == 1)
            {
                // found set with single op_id
                let single_op_id = *single_op_id.iter().next().unwrap();
                found = Some((single_op_id, *index));
            }

            if let Some((op_id, index)) = found.take() {
                id_map.insert(op_id, index);
                // remove index
                opcode_to_indices.values_mut().for_each(|set| {
                    set.remove(&index);
                });
                // remove op_id
                index_to_opcodes.values_mut().for_each(|set| {
                    set.remove(&op_id);
                });
            } else {
                panic!("could not fully reduce");
            }
        }
        id_map
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_16.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, 642);

    let result_part2 = challenge.solution_part_2();
    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, 481);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_16() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_16_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_16 part 1: {result_part1}");
        assert_eq!(result_part1, 1);

        // no example for part 2

        Ok(())
    }
}
