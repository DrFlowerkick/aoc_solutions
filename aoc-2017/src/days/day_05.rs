//!day_05.rs

use anyhow::Result;

struct ChallengeInput {
    jumps: Vec<i64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            jumps: value.lines().filter_map(|j| j.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut jumps = self.jumps.clone();
        let mut counter = 0;
        let mut index: i64 = 0;
        let num_jumps = self.jumps.len() as i64;

        loop {
            counter += 1;
            let new_index = index + jumps[index as usize];
            if new_index < 0 || new_index >= num_jumps {
                break;
            }
            jumps[index as usize] += 1;
            index = new_index;
        }
        counter
    }
    fn solution_part_2(&self) -> u64 {
        let mut jumps = self.jumps.clone();
        let mut counter = 0;
        let mut index: i64 = 0;
        let num_jumps = self.jumps.len() as i64;

        loop {
            counter += 1;
            let jump = jumps[index as usize];
            let new_index = index + jump;
            if new_index < 0 || new_index >= num_jumps {
                break;
            }
            jumps[index as usize] += if jump >= 3 { -1 } else { 1 };
            index = new_index;
        }
        counter
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_05.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 358_309);

    let result_part2 = challenge.solution_part_2();
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 28_178_177);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_05() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_05_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        let result_part2 = example.solution_part_2();
        println!("result day_05 part 2: {result_part2}");
        assert_eq!(result_part2, 10);

        Ok(())
    }
}
