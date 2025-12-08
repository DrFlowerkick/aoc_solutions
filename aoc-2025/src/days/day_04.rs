//!day_04.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::HashSet;

struct ChallengeInput {
    paper_rolls: HashSet<Point>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut paper_rolls = HashSet::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    paper_rolls.insert((x as i64, y as i64).into());
                }
            }
        }
        ChallengeInput { paper_rolls }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        self.remove_paper_roles()
    }
    fn solution_part_2(&mut self, mut num_removed_paper_rolls: usize) -> usize {
        loop {
            let num_removed_paper_rolls_increment = self.remove_paper_roles();
            num_removed_paper_rolls += num_removed_paper_rolls_increment;
            if num_removed_paper_rolls_increment == 0 {
                return num_removed_paper_rolls;
            }
        }
    }
    fn remove_paper_roles(&mut self) -> usize {
        let rolls_to_remove: HashSet<Point> = self
            .paper_rolls
            .iter()
            .filter(|p| {
                [
                    (1_i64, 0_i64),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                ]
                .into_iter()
                .filter_map(|n| self.paper_rolls.get(&p.add(n)))
                .count()
                    < 4
            })
            .copied()
            .collect();
        self.paper_rolls = self
            .paper_rolls
            .symmetric_difference(&rolls_to_remove)
            .copied()
            .collect();
        rolls_to_remove.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_04.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 1_433);

    let result_part2 = challenge.solution_part_2(result_part1);
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 8_616);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_04_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, 13);

        let result_part2 = example.solution_part_2(result_part1);
        println!("result day_04 part 2: {result_part2}");
        assert_eq!(result_part2, 43);

        Ok(())
    }
}
