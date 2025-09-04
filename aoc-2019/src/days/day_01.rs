//!day_01.rs

use anyhow::Result;

struct ChallengeInput {
    numbers: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.lines().filter_map(|n| n.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.numbers.iter().map(|n| self.calc_fuel(*n, false)).sum()
    }
    fn solution_part_2(&self) -> u64 {
        self.numbers.iter().map(|n| self.calc_fuel(*n, true)).sum()
    }
    fn calc_fuel(&self, mut weight: u64, do_loop: bool) -> u64 {
        let mut fuel = 0;
        loop {
            weight /= 3;
            if weight < 3 {
                return fuel;
            }
            weight -= 2;
            fuel += weight;
            if !do_loop {
                break;
            }
        }
        fuel
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_01.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, 3_553_700);

    let result_part2 = challenge.solution_part_2();
    println!("result day_01 part 2: {result_part2}");
    assert_eq!(result_part2, 5_327_664);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_01() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_01_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_01 part 1: {result_part1}");
        assert_eq!(result_part1, 34_241);

        let result_part2 = example.solution_part_2();
        println!("result day_01 part 2: {result_part2}");
        assert_eq!(result_part2, 2 + 2 + 966 + 50346);

        Ok(())
    }
}
