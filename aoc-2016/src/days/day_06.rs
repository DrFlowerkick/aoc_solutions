//!day_06.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    code: Vec<Vec<char>>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut code = Vec::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    code.push(vec![]);
                }
                code[x].push(c);
            }
        }
        ChallengeInput { code }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (String, String) {
        let mut letter_counters: Vec<HashMap<char, u64>> = vec![HashMap::new(); self.code.len()];
        for (x, col) in self.code.iter().enumerate() {
            for c in col.iter() {
                letter_counters[x]
                    .entry(*c)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
        let most = letter_counters
            .iter()
            .map(|lc| {
                let max = lc.values().max().unwrap();
                let c = lc.iter().find(|(_, v)| *v == max).unwrap().0;
                *c
            })
            .collect();
        let least = letter_counters
            .iter()
            .map(|lc| {
                let min = lc.values().min().unwrap();
                let c = lc.iter().find(|(_, v)| *v == min).unwrap().0;
                *c
            })
            .collect();
        (most, least)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_06.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, "agmwzecr");

    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, "owlaxqvq");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_06_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, "easter");

        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, "advent");

        Ok(())
    }
}
