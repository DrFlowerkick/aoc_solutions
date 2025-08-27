//!day_17.rs

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Adv(u128),
    Bxl(u128),
    Bst(u128),
    Jnz(usize),
    Bxc,
    Out(u128),
    Bdv(u128),
    Cdv(u128),
}

impl From<(u128, u128)> for Instruction {
    fn from(value: (u128, u128)) -> Self {
        match value.0 {
            0 => {
                // Combo
                assert!(value.1 < 7);
                Self::Adv(value.1)
            }
            1 => {
                // Literal
                assert!(value.1 < 8);
                Self::Bxl(value.1)
            }
            2 => {
                // Combo
                assert!(value.1 < 7);
                Self::Bst(value.1)
            }
            3 => {
                // Literal
                assert!(value.1 < 8);
                // only even numbers, since we jump to
                assert_eq!(value.1 & 1, 0);
                // Since I place the operand inside the instruction, my program has halve the length
                // therefore the instruction pointer of the jump must be divided by 2
                Self::Jnz(value.1 as usize / 2)
            }
            4 => {
                // Unused value.1, still checking 3-bit-size
                assert!(value.1 < 8);
                Self::Bxc
            }
            5 => {
                // Combo
                assert!(value.1 < 7);
                Self::Out(value.1)
            }
            6 => {
                // Combo
                assert!(value.1 < 7);
                Self::Bdv(value.1)
            }
            7 => {
                // Combo
                assert!(value.1 < 7);
                Self::Cdv(value.1)
            }
            _ => panic!("Only numbers < 8 are allowed."),
        }
    }
}

impl Instruction {
    fn apply(&self, reg: &mut Registers, out: &mut Vec<String>) -> Option<usize> {
        match self {
            Instruction::Adv(op) => {
                // is equal to: reg.a /= 2_u128.pow(reg.combo_operator(*op));
                reg.a >>= reg.combo_operator(*op);
            }
            Instruction::Bxl(op) => {
                reg.b ^= op;
            }
            Instruction::Bst(op) => {
                // is equal to: reg.b = reg.combo_operator(*op) % 8;
                reg.b = reg.combo_operator(*op) & 7;
            }
            Instruction::Jnz(op) => {
                if reg.a != 0 {
                    return Some(*op);
                }
            }
            Instruction::Bxc => {
                reg.b ^= reg.c;
            }
            Instruction::Out(op) => {
                // is equal to: op_mod = reg.combo_operator(*op) % 8;
                let op_mod = reg.combo_operator(*op) & 7;
                out.push(op_mod.to_string());
            }
            Instruction::Bdv(op) => {
                // is equal to: reg.b = reg.a / 2_u128.pow(reg.combo_operator(*op));
                reg.b = reg.a >> reg.combo_operator(*op);
            }
            Instruction::Cdv(op) => {
                // is equal to: reg.c = reg.a / 2_u128.pow(reg.combo_operator(*op));
                reg.c = reg.a >> reg.combo_operator(*op);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Registers {
    a: u128,
    b: u128,
    c: u128,
}

impl From<&str> for Registers {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"Register ([A-C]): (\d+)").unwrap();

        let mut registers = Registers { a: 0, b: 0, c: 0 };

        for line in value.lines() {
            if let Some(caps) = re.captures(line) {
                let register_name = &caps[1];
                let reg = caps[2].parse::<u128>().unwrap();

                match register_name {
                    "A" => registers.a = reg,
                    "B" => registers.b = reg,
                    "C" => registers.c = reg,
                    _ => unreachable!(),
                }
            }
        }
        registers
    }
}

impl Registers {
    fn combo_operator(&self, operand: u128) -> u128 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("Combo operator is always < 7."),
        }
    }
}

#[derive(Debug)]
struct Day17Data {
    reg: Registers,
    ins: Vec<Instruction>,
    part_2: Vec<u128>,
}

impl From<&str> for Day17Data {
    fn from(value: &str) -> Self {
        let (reg, ins) = value.split_once("\n\n").unwrap();
        let reg = Registers::from(reg);
        let (_, ins) = ins.split_once(':').unwrap();
        let part_2: Vec<u128> = ins
            .trim()
            .split(',')
            .map(|d| d.parse::<u128>().unwrap())
            .collect();
        let ins: Vec<Instruction> = part_2
            .chunks(2)
            .map(|c| Instruction::from((c[0], c[1])))
            .collect();
        Self { reg, ins, part_2 }
    }
}

impl Day17Data {
    fn execute(&mut self) -> String {
        let mut index = 0;
        let mut out: Vec<String> = Vec::new();
        while index < self.ins.len() {
            if let Some(jump_index) = self.ins[index].apply(&mut self.reg, &mut out) {
                index = jump_index;
            } else {
                index += 1;
            }
        }
        out.join(",")
    }
    // some reverse engineering, what is really happening
    // Bst(4) -> b = a & 7 (grab first 3 bits of a; 7 = 111)
    // Bxl(6) -> b = b ^ 6 (6 = 110)
    // Cdv(5) -> c = a >> b (shift b bits to the right; only first 3 bits are relevant because of Out(5), s. below)
    // Bxc    -> b = b ^ c
    // Bxl(7) -> b = b ^ 7 (7 = 111)
    // Adv(3) -> a = a >> 3 (shift 3 bits to the right)
    // Out(5) -> out = b & 7 (grab first 3 bits of b and collect them for output; 7 = 111)
    // Jnz(0) -> end program if a == 0
    // To find the minimum value for a, which results in an output similar to program input, we start from
    // the back and reverse the process above
    fn reverse_solution(&self) -> u128 {
        let min_a = u128::MAX;
        if let Some(final_a) = self.recursive_solver(self.part_2.len() - 1, 0, min_a) {
            final_a
        } else {
            panic!("No Solution!");
        }
    }
    fn recursive_solver(&self, index: usize, a: u128, mut min_a: u128) -> Option<u128> {
        let out = self.part_2[index];
        let new_a = a << 3;
        let mut is_solved = false;
        for b in 0..8_u128 {
            let new_a = new_a + b;
            let b_bxl_6 = b ^ 6;
            let c = (new_a >> b_bxl_6) & 7;
            let check_out = b_bxl_6 ^ c ^ 7;
            if out == check_out {
                // possible solution
                if index == 0 {
                    // end of recursive call
                    if new_a < min_a {
                        is_solved = true;
                        min_a = new_a;
                    }
                } else if let Some(final_a) = self.recursive_solver(index - 1, new_a, min_a) {
                    if final_a < min_a {
                        is_solved = true;
                        min_a = final_a;
                    }
                }
            }
        }
        is_solved.then_some(min_a)
    }
}

pub fn day_17() -> Result<()> {
    // a part 1: 37293246
    let input = include_str!("../../../../aoc_input/aoc-2024/day_17.txt");
    let mut challenge = Day17Data::from(input);

    let result_part1 = challenge.execute();
    println!("result day 17 part 1: {}", result_part1);
    assert_eq!(result_part1, "1,5,0,1,7,4,1,0,3");

    let result_part2 = challenge.reverse_solution();
    println!("result day 17 part 2: {}", result_part2);
    assert_eq!(result_part2, 47_910_079_998_866);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_17_example.txt");
        let mut challenge = Day17Data::from(input);

        let result_part1 = challenge.execute();
        println!("result day 17 part 1: {}", result_part1);
        assert_eq!(result_part1, "4,6,3,5,6,3,5,2,1,0");
        /*
        Example of part two does not really help to solve this challenge.
        Therefore I skipped it.
        */
        Ok(())
    }
}
