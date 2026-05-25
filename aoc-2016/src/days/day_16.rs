//!day_16.rs

use anyhow::Result;

struct ChallengeInput {
    bits: Vec<bool>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            bits: value.chars().map(|c| c == '1').collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, len: usize) -> String {
        let mut disk_bits: Vec<bool> = Vec::with_capacity(len);
        disk_bits.extend_from_slice(&self.bits);
        // first collect bits until >= len
        while disk_bits.len() < len {
            disk_bits = disk_bits
                .iter()
                .copied()
                .chain([false])
                .chain(disk_bits.iter().rev().map(|b| !*b))
                .collect();
        }
        // strip unneeded bits
        disk_bits.truncate(len);
        // than calc checksum
        while disk_bits.len() & 1 == 0 {
            disk_bits = disk_bits.chunks(2).map(|c| c[0] == c[1]).collect();
        }
        // return bits as chars
        disk_bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect()
    }
    fn solution_part_2(&self) -> String {
        self.solution_part_1(35_651_584)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_16.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(272);
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, "11100111011101111");

    let result_part2 = challenge.solution_part_2();
    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, "10001110010000110");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_16() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_16_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(20);
        println!("result day_16 part 1: {result_part1}");
        assert_eq!(result_part1, "01100");

        Ok(())
    }
}
