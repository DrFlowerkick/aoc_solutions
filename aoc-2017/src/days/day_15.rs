//!day_15.rs

use anyhow::Result;

#[derive(Clone, Copy)]
struct Generator {
    value: u64,
    factor: u64,
}

impl Eq for Generator {}

impl PartialEq for Generator {
    fn eq(&self, other: &Self) -> bool {
        // 16 bit 1
        let mask: u64 = 256 * 256 - 1;
        self.value & mask == other.value & mask
    }
}

impl Generator {
    fn new_a(value: u64) -> Self {
        Self {
            value,
            factor: 16_807,
        }
    }
    fn new_b(value: u64) -> Self {
        Self {
            value,
            factor: 48_271,
        }
    }
    fn one_cycle(&mut self) {
        self.value = (self.value * self.factor).rem_euclid(2_147_483_647);
    }
    fn cycle_until(&mut self, multiple: u64) {
        // 16 bit 1
        let mask: u64 = 256 * 256 - 1;
        while {
            self.one_cycle();
            !(self.value & mask).is_multiple_of(multiple)
        } {}
    }
}

struct ChallengeInput {
    start_a: u64,
    start_b: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let start: Vec<u64> = value
            .lines()
            .filter_map(|l| {
                let (_, s) = l.split_once(" starts with ").unwrap();
                s.parse().ok()
            })
            .collect();
        ChallengeInput {
            start_a: start[0],
            start_b: start[1],
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut a = Generator::new_a(self.start_a);
        let mut b = Generator::new_b(self.start_b);
        let mut count = 0;
        for _ in 0..40_000_000 {
            a.one_cycle();
            b.one_cycle();
            if a == b {
                count += 1;
            }
        }
        count
    }
    fn solution_part_2(&self) -> u64 {
        let mut a = Generator::new_a(self.start_a);
        let mut b = Generator::new_b(self.start_b);
        let mut count = 0;
        for _ in 0..5_000_000 {
            a.cycle_until(4);
            b.cycle_until(8);
            if a == b {
                count += 1;
            }
        }
        count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_15.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 600);

    let result_part2 = challenge.solution_part_2();
    println!("result day_15 part 2: {result_part2}");
    assert_eq!(result_part2, 313);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_15() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_15_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_15 part 1: {result_part1}");
        assert_eq!(result_part1, 588);

        let result_part2 = example.solution_part_2();
        println!("result day_15 part 2: {result_part2}");
        assert_eq!(result_part2, 309);

        Ok(())
    }
}
