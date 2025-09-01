//!day_23.rs
// ############################################################################
// # WARNING: this takes a lot of time to execute, see below in fn solution() #
// # I keep this stuff just for reference.                                    #
// ############################################################################

use anyhow::Result;
use std::fmt::Write;
use std::time::Instant;

struct ChallengeInput {
    cups: Vec<u64>,
    min_cup: u64,
    max_cup: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let cups: Vec<u64> = value
            .chars()
            .filter_map(|d| d.to_digit(10))
            .map(|d| d as u64)
            .collect();
        let min_cup = *cups.iter().min().unwrap();
        let max_cup = *cups.iter().max().unwrap();
        ChallengeInput {
            cups,
            min_cup,
            max_cup,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> String {
        self.do_moves(100);
        let index_1 = self.cups.iter().position(|c| *c == 1).unwrap();
        self.cups.rotate_left(index_1);
        let mut cups = String::new();
        self.cups.iter().skip(1).for_each(|c| {
            write!(&mut cups, "{c}").unwrap();
        });
        cups
    }
    fn solution_part_2(&mut self) -> u64 {
        self.cups.extend(self.max_cup + 1..=1_000_000);
        self.max_cup = 1_000_000;
        self.do_moves(10_000_000);
        let index_1 = self.cups.iter().position(|c| *c == 1).unwrap();
        self.cups[index_1 + 1] * self.cups[index_1 + 2]
    }
    fn do_moves(&mut self, moves: usize) {
        let mut current_index = 0;
        let start = Instant::now();
        for current_move in 0..moves {
            if current_move > 0 && current_move % 100_000 == 0 {
                let elapsed = start.elapsed();
                println!("{current_move}");
                println!("elapsed: {:?}", elapsed);
                let estimation_remaining_duration = elapsed
                    .checked_div(current_move as u32)
                    .unwrap()
                    .checked_mul((moves - current_move) as u32)
                    .unwrap();
                println!("estimation: {:?}", estimation_remaining_duration);
            }
            let index_offset =
                if current_index == self.cups.len() - 1 || current_index + 3 < self.cups.len() {
                    // pick up does not run over end of vec
                    self.handle_pick_up(current_index)
                } else {
                    /*
                    // pick up does run over end of vec -> rotate left, handle pick up, rotate back right
                    self.cups.rotate_left(3);
                    self.handle_pick_up(current_index - 3);
                    self.cups.rotate_right(3);
                     */
                    // rotate as much right, that current index is zero again
                    let rot_right = self.cups.len() - current_index;
                    self.cups.rotate_right(rot_right);
                    current_index = 0;
                    self.handle_pick_up(current_index)
                };
            current_index += 1 + index_offset;
            if current_index >= self.cups.len() {
                current_index -= self.cups.len();
            }
        }
        let elapsed = start.elapsed();
        println!("elapsed: {:?}", elapsed);
    }
    fn handle_pick_up(&mut self, current_index: usize) -> usize {
        let start_index = (current_index + 1) % self.cups.len();
        let end_index = (current_index + 3) % self.cups.len();
        let blocked_destination = &self.cups[start_index..=end_index];
        let destination_index =
            self.get_destination_index(self.cups[current_index], blocked_destination);
        if destination_index > start_index {
            // rotate left inklusive destination_index
            self.cups[start_index..=destination_index].rotate_left(3);
            0
        } else {
            // rotate right exklusive destination_index ...
            self.cups[destination_index + 1..=end_index].rotate_right(3);
            // ... than rotate all cups left for current cup moving back to current index
            //self.cups.rotate_left(3);
            3
        }
    }
    fn get_destination_index(&self, mut cup: u64, pick_up_slice: &[u64]) -> usize {
        loop {
            cup = if cup == self.min_cup {
                self.max_cup
            } else {
                cup - 1
            };
            if !pick_up_slice.contains(&cup) {
                break;
            }
        }
        self.cups.iter().position(|c| *c == cup).unwrap()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_23.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, "65432978");

    // part two takes with this "slice brute force" about 30 minutes on my notebook
    // the reason for this is probably fn get_destination_index(), because it must iter
    // through a million values to find destination. Do the 10 million times and it takes some time...
    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 287_230_227_046);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_23_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1 (brute force): {result_part1}");
        assert_eq!(result_part1, "67384529");

        /* THIS WILL TAKE 30 MiNUTES WITH RELEASE
        let mut example = ChallengeInput::from(input);
        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        assert_eq!(result_part2, 149_245_887_792);
        */

        Ok(())
    }
}
