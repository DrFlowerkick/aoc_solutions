//!day_10.rs

use anyhow::{Result, anyhow};
use good_lp::*;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Machine {
    indicator_lights: u64,
    buttons: Vec<u64>,
    joltage_requirements: Vec<u64>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let (indicator_lights, remaining) = value.split_once("] ").unwrap();
        let (buttons, joltage_requirements) = remaining.split_once(" {").unwrap();
        Machine {
            indicator_lights: indicator_lights[1..].chars().enumerate().fold(
                0,
                |current, (bit, c)| {
                    if c == '#' {
                        current | 1 << bit
                    } else {
                        current
                    }
                },
            ),
            buttons: buttons
                .split_whitespace()
                .map(|b| {
                    b.trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|b| b.parse::<u32>().unwrap())
                        .fold(0, |current, bit| current | 1 << bit)
                })
                .collect(),
            joltage_requirements: joltage_requirements
                .trim_end_matches('}')
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect(),
        }
    }
}

impl Machine {
    fn find_num_buttons_for_indicator_lights(&self) -> u64 {
        let mut btree_queue: BTreeSet<(u64, u64)> = BTreeSet::new();
        btree_queue.insert((0, 0));
        while let Some((count, current_indicators)) = btree_queue.pop_first() {
            if current_indicators == self.indicator_lights {
                return count;
            }
            let mask = current_indicators ^ self.indicator_lights;
            for button in self.buttons.iter().filter(|b| **b & mask > 0) {
                btree_queue.insert((count + 1, current_indicators ^ *button));
            }
        }
        0
    }
    fn _find_num_buttons_for_joltage_requirements(&self) -> u64 {
        // this works, but it is too slow for bigger numbers -> we need a better solution
        let mut btree_queue: BTreeSet<(u64, u64, u64, Vec<u64>)> = BTreeSet::new();
        let all_one = 2_u64.pow(self.joltage_requirements.len() as u32) - 1;
        btree_queue.insert((
            0,
            self.joltage_requirements.iter().sum(),
            all_one,
            self.joltage_requirements.clone(),
        ));
        while let Some((count, remaining_joltage, count_mask, current_joltage)) =
            btree_queue.pop_first()
        {
            if count_mask == 0 {
                return count;
            }
            for button in self.buttons.iter().filter(|b| **b & count_mask == **b) {
                let mut next_mask = count_mask;
                let mut next_remaining = remaining_joltage;
                let mut next_joltage = current_joltage.clone();
                for (index, nj) in next_joltage.iter_mut().enumerate() {
                    let bit = 1_u64 << index;
                    if button & bit == bit {
                        *nj -= 1;
                        next_remaining -= 1;
                        if *nj == 0 {
                            // deactivate bit
                            next_mask &= all_one - bit;
                        }
                    }
                }
                btree_queue.insert((count + 1, next_remaining, next_mask, next_joltage));
            }
        }
        0
    }
    /// Part two is in fact a collection of linear expressions. The target value of each
    /// expression is the respective joltage requirement. The variables are the buttons,
    /// which trigger the joltage requirement. For the first machine of the example it looks
    /// like this:
    /// b0 + b1 +      b3           = 3
    ///           b2 + b3 + b4      = 4
    ///      b1 +                b5 = 5
    ///                     b4 + b5 = 3
    /// Since there are more variables than expressions, we get multiple solutions.
    /// We want to find the solution with the least number of button presses, which
    /// is equal to the minimum of the sum of all variables.
    fn find_num_buttons_for_joltage_requirements(&self) -> Result<u64> {
        // variables
        let mut vars = variables!();
        // each button is a variable (integer, min 0)
        let x = vars.add_vector(variable().integer().min(0), self.buttons.len());

        // target function: minimize sum of all button presses
        let objective: Expression = x.iter().sum();
        let mut problem = vars.minimise(objective).using(default_solver);

        // constraints: define expression for each joltage requirement
        for (bit_index, &target_value) in self.joltage_requirements.iter().enumerate() {
            let bit_mask = 1_u64 << bit_index;

            // collect all variables (Buttons), which have 1 at current bit position
            let sum_of_buttons_for_this_bit: Expression = self
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, btn_mask)| (**btn_mask & bit_mask) != 0)
                .map(|(btn_idx, _)| x[btn_idx])
                .sum();

            // the sum of buttons, which have 1 at current bit position
            // has to be equal to target value
            problem = problem.with(sum_of_buttons_for_this_bit.eq(target_value as f64));
        }

        // solve
        match problem.solve() {
            Ok(solution) => {
                // sum of all variables equals total number of button presses
                let result: f64 = x.iter().map(|v| solution.value(*v)).sum();
                Ok(result.round() as u64)
            }
            Err(e) => Err(anyhow!("No solution for machine: {:?}", e)),
        }
    }
}

#[derive(Debug)]
struct ChallengeInput {
    machines: Vec<Machine>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            machines: value.lines().map(Machine::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.machines
            .iter()
            .map(|m| m.find_num_buttons_for_indicator_lights())
            .sum()
    }
    fn solution_part_2(&self) -> Result<u64> {
        let mut total_count = 0;
        for machine in self.machines.iter() {
            total_count += machine.find_num_buttons_for_joltage_requirements()?;
        }
        Ok(total_count)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_10.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 488);

    let result_part2 = challenge.solution_part_2()?;
    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 18_771);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_10_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_10 part 1: {result_part1}");
        assert_eq!(result_part1, 7);

        let result_part2 = example.solution_part_2()?;
        println!("result day_10 part 2: {result_part2}");
        assert_eq!(result_part2, 33);

        Ok(())
    }
}
