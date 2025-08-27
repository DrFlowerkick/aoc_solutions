//!day_10.rs

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
    fn solution_part_1(&mut self) -> u64 {
        let mut count_dif_1 = 0;
        let mut count_dif_3 = 0;
        self.numbers.push(0);
        self.numbers.sort();
        let max = *self.numbers.last().unwrap();
        self.numbers.push(max + 3);
        for (i, n) in self.numbers[..self.numbers.len() - 1].iter().enumerate() {
            match self.numbers[i + 1] - *n {
                3 => count_dif_3 += 1,
                2 => (),
                1 => count_dif_1 += 1,
                _ => panic!("unexpected difference"),
            }
        }
        count_dif_1 * count_dif_3
    }
    fn solution_part_2(&self) -> u64 {
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_10.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 1_836);

    let result_part2 = challenge.solution_part_2();
    println!("result day_10 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_10_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_10 part 1: {result_part1}");
        assert_eq!(result_part1, 220);

        let result_part2 = example.solution_part_2();
        println!("result day_10 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
