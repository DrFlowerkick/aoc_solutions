//!day_08.rs

use anyhow::Result;

struct ChallengeInput {
    numbers: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut index = 0;
        self.sum_of_metadata(&mut index)
    }
    fn solution_part_2(&self) -> u64 {
        let mut index = 0;
        self.node_values(&mut index)
    }
    fn sum_of_metadata(&self, index: &mut usize) -> u64 {
        let num_children = self.numbers[*index];
        let num_metadata = self.numbers[*index + 1];
        *index += 2;
        let mut metadata = 0;
        for _ in 0..num_children {
            metadata += self.sum_of_metadata(index);
        }
        metadata += self.numbers[*index..*index + num_metadata as usize]
            .iter()
            .sum::<u64>();
        *index += num_metadata as usize;
        metadata
    }
    fn node_values(&self, index: &mut usize) -> u64 {
        let num_children = self.numbers[*index];
        let num_metadata = self.numbers[*index + 1];
        *index += 2;
        let child_values: Vec<u64> = (0..num_children).map(|_| self.node_values(index)).collect();
        let node_value = if child_values.is_empty() {
            self.numbers[*index..*index + num_metadata as usize]
                .iter()
                .sum()
        } else {
            self.numbers[*index..*index + num_metadata as usize]
                .iter()
                .filter_map(|ci| ci.checked_sub(1))
                .filter_map(|ci| child_values.get(ci as usize))
                .sum()
        };
        *index += num_metadata as usize;
        node_value
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_08.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 44_838);

    let result_part2 = challenge.solution_part_2();
    println!("result day_08 part 2: {result_part2}");
    assert_eq!(result_part2, 22_198);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_08_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 138);

        let result_part2 = example.solution_part_2();
        println!("result day_08 part 2: {result_part2}");
        assert_eq!(result_part2, 66);

        Ok(())
    }
}
