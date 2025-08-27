//!day_11.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;
use std::collections::VecDeque;

#[derive(Clone, Copy, Default)]
struct EnergyLevel(u32);

impl From<char> for EnergyLevel {
    fn from(value: char) -> Self {
        EnergyLevel(value.to_digit(10).unwrap())
    }
}

impl EnergyLevel {
    fn increment(&mut self) -> bool {
        let will_flash = self.0 == 9;
        if self.0 < 10 {
            self.0 += 1;
        }
        will_flash
    }
    fn flash(&mut self) -> bool {
        if self.0 != 10 {
            return false;
        }
        self.0 = 11;
        true
    }
    fn reset_flashed(&mut self) -> bool {
        if self.0 < 10 {
            return false;
        }
        assert!(self.0 == 11);
        self.0 = 0;
        true
    }
}

struct ChallengeInput {
    octopuses: MyMap2D<EnergyLevel, 10, 10>,
    first_full_flash: Option<usize>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            octopuses: MyMap2D::from(value),
            first_full_flash: None,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        (1..=100)
            .map(|current_step| self.one_step(current_step))
            .sum()
    }
    fn solution_part_2(&mut self) -> usize {
        let mut current_step = 101;
        while self.first_full_flash.is_none() {
            self.one_step(current_step);
            current_step += 1;
        }
        self.first_full_flash.unwrap()
    }
    fn one_step(&mut self, current_step: usize) -> usize {
        let mut flash_queue: VecDeque<_> = self
            .octopuses
            .iter_mut()
            .map(|(p, o)| (p, o.increment()))
            .filter(|(_, will_flash)| *will_flash)
            .map(|(p, _)| p)
            .collect();
        while let Some(octopus) = flash_queue.pop_front() {
            assert!(self.octopuses.get_mut(octopus).flash());
            let neighbors: Vec<_> = self
                .octopuses
                .iter_neighbors_with_corners(octopus)
                .map(|(p, _, _)| p)
                .collect();
            for neighbor_octopus in neighbors {
                if self.octopuses.get_mut(neighbor_octopus).increment() {
                    flash_queue.push_back(neighbor_octopus);
                }
            }
        }
        let num_flashes = self
            .octopuses
            .iter_mut()
            .map(|(_, o)| o.reset_flashed())
            .filter(|has_flashed| *has_flashed)
            .count();
        if num_flashes == 100 && self.first_full_flash.is_none() {
            self.first_full_flash = Some(current_step);
        }
        num_flashes
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_11.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, 1_649);

    let result_part2 = challenge.solution_part_2();
    println!("result day_11 part 2: {result_part2}");
    assert_eq!(result_part2, 256);

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;

    impl Display for EnergyLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_steps() {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_11_example.txt");
        let mut example = ChallengeInput::from(input);

        println!("{}", example.octopuses);

        let mut sum: usize = 0;
        for step in 1..=10 {
            sum += example.one_step(step);
            println!("After step {step}");
            println!("{}\n", example.octopuses);
        }

        println!("After 10 steps: {sum} flashes");
        assert_eq!(sum, 204);

        (11..=195).for_each(|current_step| {
            example.one_step(current_step);
        });
        println!("After step 195");
        println!("{}", example.octopuses);
        assert_eq!(example.first_full_flash, Some(195));
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_11_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_11 part 1: {result_part1}");
        assert_eq!(result_part1, 1_656);

        let result_part2 = example.solution_part_2();
        println!("result day_11 part 2: {result_part2}");
        assert_eq!(result_part2, 195);

        Ok(())
    }
}
