//!day_24.rs

use anyhow::Result;
use my_lib::my_algo_collection::collect_all_n_from_m_elements;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Group {
    num_packages: u64,
    quantum_entanglement: u64,
    packages: Vec<u64>,
}

struct ChallengeInput {
    weights: HashSet<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            weights: value.lines().filter_map(|v| v.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, num_groups: u64) -> u64 {
        let group_weight: u64 = self.weights.iter().sum::<u64>() / num_groups;
        let possible_solutions = self.find_possible_solutions(&[], Some(7), group_weight);
        let solution = possible_solutions
            .iter()
            .find(|ps| {
                let blocked_weights: Vec<u64> = ps.packages.to_vec();
                self.check_remaining_groups(&blocked_weights, num_groups - 1, group_weight)
            })
            .unwrap();
        solution.packages.iter().product()
    }
    fn find_possible_solutions(
        &self,
        blocked_weights: &[u64],
        max_num: Option<usize>,
        group_weight: u64,
    ) -> BTreeSet<Group> {
        let mut possible_solutions: BTreeSet<Group> = BTreeSet::new();
        let remaining_weights: Vec<u64> = self
            .weights
            .difference(&blocked_weights.iter().copied().collect())
            .copied()
            .collect();
        let max_n = if let Some(mn) = max_num {
            mn
        } else {
            remaining_weights.len() - 1
        };
        for n in 1..max_n {
            for n_weights in collect_all_n_from_m_elements(&remaining_weights, n)
                .iter()
                .filter(|g| g.iter().sum::<u64>() == group_weight)
            {
                let group = Group {
                    num_packages: n_weights.len() as u64,
                    quantum_entanglement: n_weights.iter().product(),
                    packages: n_weights.clone(),
                };
                possible_solutions.insert(group);
            }
        }
        possible_solutions
    }
    fn check_remaining_groups(
        &self,
        blocked_weights: &[u64],
        remaining_groups: u64,
        group_weight: u64,
    ) -> bool {
        if remaining_groups == 1 {
            self.weights
                .difference(&blocked_weights.iter().copied().collect())
                .sum::<u64>()
                == group_weight
        } else {
            self.find_possible_solutions(blocked_weights, None, group_weight)
                .iter()
                .any(|ps| {
                    let bw: Vec<u64> = blocked_weights
                        .iter()
                        .chain(ps.packages.iter())
                        .copied()
                        .collect();
                    self.check_remaining_groups(&bw, remaining_groups - 1, group_weight)
                })
        }
    }
    fn solution_part_2(&self) -> u64 {
        self.solution_part_1(4)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_24.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(3);
    println!("result day_24 part 1: {result_part1}");
    assert_eq!(result_part1, 11_266_889_531);

    let result_part2 = challenge.solution_part_2();
    println!("result day_24 part 2: {result_part2}");
    assert_eq!(result_part2, 77_387_711);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_sum_is_multiple_of_three() {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_24_example.txt");
        let example = ChallengeInput::from(input);

        let sum_example: u64 = example.weights.iter().sum();
        //dbg!(sum_example / 3);
        assert_eq!(sum_example % 3, 0);
        assert_eq!(sum_example % 4, 0);

        let input = include_str!("../../../../aoc_input/aoc-2015/day_24.txt");
        let challenge = ChallengeInput::from(input);

        let sum_challenge: u64 = challenge.weights.iter().sum();
        //dbg!(sum_challenge / 3);
        assert_eq!(sum_challenge % 3, 0);
        assert_eq!(sum_challenge % 4, 0);
    }

    #[test]
    fn test_example_day_24() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_24_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(3);
        println!("result day_24 part 1: {result_part1}");
        assert_eq!(result_part1, 99);

        let result_part2 = example.solution_part_2();
        println!("result day_24 part 2: {result_part2}");
        assert_eq!(result_part2, 44);

        Ok(())
    }
}
