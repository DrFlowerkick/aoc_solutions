//!day_22.rs

use anyhow::Result;
use std::collections::{HashSet, VecDeque};

struct ChallengeInput {
    player_1: VecDeque<usize>,
    player_2: VecDeque<usize>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (player_1, player_2) = value.split_once("\n\n").unwrap();
        ChallengeInput {
            player_1: player_1
                .lines()
                .skip(1)
                .filter_map(|num| num.parse().ok())
                .collect(),
            player_2: player_2
                .lines()
                .skip(1)
                .filter_map(|num| num.parse().ok())
                .collect(),
        }
    }
}

impl From<(&[usize], &[usize])> for ChallengeInput {
    fn from(value: (&[usize], &[usize])) -> Self {
        ChallengeInput {
            player_1: value.0.iter().copied().collect(),
            player_2: value.1.iter().copied().collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            let p1 = self.player_1.pop_front().unwrap();
            let p2 = self.player_2.pop_front().unwrap();
            if p1 > p2 {
                self.player_1.push_back(p1);
                self.player_1.push_back(p2);
            } else {
                self.player_2.push_back(p2);
                self.player_2.push_back(p1);
            }
        }
        let winner = if self.player_2.is_empty() {
            &self.player_1
        } else {
            &self.player_2
        };
        winner
            .iter()
            .enumerate()
            .map(|(i, num)| (winner.len() - i) * *num)
            .sum()
    }
    fn solution_part_2(&mut self) -> usize {
        let winner = if self.play_recursive() {
            &self.player_1
        } else {
            &self.player_2
        };
        winner
            .iter()
            .enumerate()
            .map(|(i, num)| (winner.len() - i) * *num)
            .sum()
    }
    fn play_recursive(&mut self) -> bool {
        let mut cache: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            let cache_entry = (self.player_1.clone(), self.player_2.clone());
            if cache.contains(&cache_entry) {
                // player 1 one
                return true;
            }
            cache.insert(cache_entry);
            let p1 = self.player_1.pop_front().unwrap();
            let p2 = self.player_2.pop_front().unwrap();
            let winner = if self.player_1.len() >= p1 && self.player_2.len() >= p2 {
                let mut sub_game = Self::from((
                    &self.player_1.make_contiguous()[0..p1],
                    &self.player_2.make_contiguous()[0..p2],
                ));
                sub_game.play_recursive()
            } else {
                p1 > p2
            };
            if winner {
                self.player_1.push_back(p1);
                self.player_1.push_back(p2);
            } else {
                self.player_2.push_back(p2);
                self.player_2.push_back(p1);
            }
        }
        // player 1 wins, if player 2 has no cards left
        self.player_2.is_empty()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_22.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 33_680);

    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 33_683);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_22_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_22 part 1: {result_part1}");
        assert_eq!(result_part1, 306);

        let mut example = ChallengeInput::from(input);
        let result_part2 = example.solution_part_2();
        println!("result day_22 part 2: {result_part2}");
        assert_eq!(result_part2, 291);

        Ok(())
    }
}
