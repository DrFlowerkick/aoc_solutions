//!day_16.rs

use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Instruction = (usize, usize, usize, usize);

#[derive(Debug, Clone)]
struct Sample {
    before: Vec<usize>,
    instruction: Instruction,
    after: Vec<usize>,
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
            before: caps[0..=3].to_vec(),
            instruction: (caps[4], caps[5], caps[6], caps[7]),
            after: caps[8..=11].to_vec(),
        }
    }
}

impl Sample {
    fn try_opcode(&self, oc: &Opcode) -> bool {
        oc.execute(
            self.instruction.1,
            self.instruction.2,
            self.instruction.3,
            self.before.clone(),
        ) == self.after
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Opcode {
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

pub const ALL_OPCODES: [Opcode; 16] = [
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

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        match value {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            _ => panic!("unknown opcode name"),
        }
    }
}

impl Opcode {
    pub fn execute(&self, a: usize, b: usize, c: usize, mut reg: Vec<usize>) -> Vec<usize> {
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
        let id_map = self.identify_opcode_ids();
        let mut register = vec![0; 4];
        for (op_id, a, b, c) in self.instructions.iter() {
            register = id_map.get(op_id).unwrap().execute(*a, *b, *c, register);
        }
        register[0]
    }
    fn identify_opcode_ids(&self) -> HashMap<usize, Opcode> {
        // Map: Opcode ID (Input) -> Possible Opcode Enums
        let mut opcode_id_to_opcodes: HashMap<usize, HashSet<Opcode>> = HashMap::new();
        // Map: Opcode Enum -> Possible Opcode IDs (Input)
        let mut opcode_to_opcode_ids: HashMap<Opcode, HashSet<usize>> = HashMap::new();

        // analyze samples
        for sample in self.samples.iter() {
            let matches: HashSet<Opcode> = ALL_OPCODES
                .into_iter()
                .filter(|oc| sample.try_opcode(oc))
                .collect();

            opcode_id_to_opcodes
                .entry(sample.instruction.0)
                .and_modify(|set| {
                    // only keep opcodes, which are in both hash sets,
                    // because valid opcodes must fit all samples with the same opcode id
                    *set = set.intersection(&matches).copied().collect();
                })
                .or_insert(matches);
        }

        // "reverse" mapping of opcode to opcode id
        for oc in ALL_OPCODES {
            opcode_to_opcode_ids.insert(oc, HashSet::new());
            for (op_id, indices) in &opcode_id_to_opcodes {
                if indices.contains(&oc) {
                    opcode_to_opcode_ids.entry(oc).and_modify(|set| {
                        set.insert(*op_id);
                    });
                }
            }
        }

        // reduce mappings by identifying single opcodes or opcode ids
        let mut id_map: HashMap<usize, Opcode> = HashMap::new();
        while id_map.len() < ALL_OPCODES.len() {
            let mut found = None;
            if let Some((op_id, single_opcode)) =
                opcode_id_to_opcodes.iter().find(|(_, set)| set.len() == 1)
            {
                // found set with single index
                let single_opcode = *single_opcode.iter().next().unwrap();
                found = Some((*op_id, single_opcode));
            } else if let Some((index, single_op_id)) =
                opcode_to_opcode_ids.iter().find(|(_, set)| set.len() == 1)
            {
                // found set with single op_id
                let single_op_id = *single_op_id.iter().next().unwrap();
                found = Some((single_op_id, *index));
            }

            // found matching pair of opcode id and opcode
            if let Some((op_id, opcode)) = found.take() {
                id_map.insert(op_id, opcode);
                // remove matched opcode from all entries
                opcode_id_to_opcodes.values_mut().for_each(|set| {
                    set.remove(&opcode);
                });
                // remove matched op_id from all entries
                opcode_to_opcode_ids.values_mut().for_each(|set| {
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
