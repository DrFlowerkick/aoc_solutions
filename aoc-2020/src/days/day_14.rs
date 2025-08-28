//!day_14.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default)]
struct BitMask {
    and: u64,
    or: u64,
    float_or: u64,
    float: u64,
}

impl From<&str> for BitMask {
    fn from(value: &str) -> Self {
        let (and, or, float_or, float) =
            value.chars().fold((0, 0, 0, 0), |(am, om, fo, f), bit| {
                let (a_bit, o_bit, fo_bit, f_bit) = match bit {
                    'X' => (1, 0, 0, 1),
                    val => {
                        let bit = val.to_digit(2).unwrap() as u64;
                        (bit, bit, bit, 0)
                    }
                };
                (
                    (am << 1) + a_bit,
                    (om << 1) + o_bit,
                    (fo << 1) + fo_bit,
                    (f << 1) + f_bit,
                )
            });
        BitMask {
            and,
            or,
            float_or,
            float,
        }
    }
}

impl BitMask {
    fn apply_to_val(&self, value: u64) -> u64 {
        (value & self.and) | self.or
    }
    fn apply_to_mem_address(&self, mem: u64) -> Vec<u64> {
        // 1. activate bits as given by non X bits
        let mem = mem | self.float_or;
        // 2. identify float bits
        let float_bit = 1;
        let mut float_bits: Vec<u64> = Vec::new();
        for shift in 0..36 {
            let shifted_fb = float_bit << shift;
            if shifted_fb & self.float == shifted_fb {
                // found float bit
                float_bits.push(shifted_fb);
            }
        }
        // 3. apply all combinations of float bits to mem and collect resulting addresses
        let max_mem: u64 = 0b111111111111111111111111111111111111;
        let mut addresses: Vec<u64> = Vec::new();
        let max_combinations = 2_usize.pow(float_bits.len() as u32);
        for bit_indices in 0..max_combinations {
            let mut float_mem = mem;
            for (float_bit_index, float_bit) in float_bits
                .iter()
                .enumerate()
                .map(|(i, v)| (2_usize.pow(i as u32), v))
            {
                if float_bit_index & bit_indices == float_bit_index {
                    // activate bit
                    float_mem |= float_bit;
                } else {
                    // deactivate bit
                    float_mem &= max_mem ^ float_bit;
                }
            }
            addresses.push(float_mem);
        }
        addresses
    }
}

enum MemAction {
    BitMask(BitMask),
    WriteToMem(u64, u64),
}

impl From<&str> for MemAction {
    fn from(value: &str) -> Self {
        let (action, value) = value.split_once(" = ").unwrap();
        if action == "mask" {
            MemAction::BitMask(BitMask::from(value))
        } else {
            let address = action.strip_prefix("mem[").unwrap();
            let address = address.strip_suffix("]").unwrap().parse().unwrap();
            MemAction::WriteToMem(address, value.parse().unwrap())
        }
    }
}

impl MemAction {
    fn apply_part1(&self, memory: &mut HashMap<u64, u64>, bit_mask: &mut BitMask) {
        match self {
            MemAction::BitMask(bm) => *bit_mask = *bm,
            MemAction::WriteToMem(add, val) => {
                let val = bit_mask.apply_to_val(*val);
                memory.entry(*add).and_modify(|v| *v = val).or_insert(val);
            }
        }
    }
    fn apply_part2(&self, memory: &mut HashMap<u64, u64>, bit_mask: &mut BitMask) {
        match self {
            MemAction::BitMask(bm) => *bit_mask = *bm,
            MemAction::WriteToMem(add, val) => {
                for mem in bit_mask.apply_to_mem_address(*add) {
                    memory.entry(mem).and_modify(|v| *v = *val).or_insert(*val);
                }
            }
        }
    }
}

struct ChallengeInput {
    mem_actions: Vec<MemAction>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            mem_actions: value.lines().map(MemAction::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut bit_mask = BitMask::default();
        for mem_action in self.mem_actions.iter() {
            mem_action.apply_part1(&mut memory, &mut bit_mask);
        }
        memory.values().sum()
    }
    fn solution_part_2(&self) -> u64 {
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut bit_mask = BitMask::default();
        for mem_action in self.mem_actions.iter() {
            mem_action.apply_part2(&mut memory, &mut bit_mask);
        }
        memory.values().sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_14.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, 3_059_488_894_985);

    let result_part2 = challenge.solution_part_2();
    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 2_900_994_392_308);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_14() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_14_example_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_14 part 1: {result_part1}");
        assert_eq!(result_part1, 165);

        let input = include_str!("../../../../aoc_input/aoc-2020/day_14_example_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_14 part 2: {result_part2}");
        assert_eq!(result_part2, 208);

        Ok(())
    }
}
