//!day_07.rs

use anyhow::Result;
use petgraph::{Direction, graphmap::DiGraphMap};
use regex::Regex;
use std::collections::{BTreeSet, HashSet};

struct ChallengeInput {
    instructions: DiGraphMap<char, ()>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut instructions: DiGraphMap<char, ()> = DiGraphMap::new();
        let re = Regex::new(r"Step ([A-Z]) .* step ([A-Z])").unwrap();
        for line in value.lines() {
            let caps = re.captures(line).unwrap();
            let before = caps[1].chars().next().unwrap();
            let after = caps[2].chars().next().unwrap();
            instructions.add_edge(before, after, ());
        }
        ChallengeInput { instructions }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> String {
        let mut seen: HashSet<char> = HashSet::new();
        let mut sorted_queue: BTreeSet<char> = self
            .instructions
            .nodes()
            .filter(|n| {
                self.instructions
                    .edges_directed(*n, Direction::Incoming)
                    .count()
                    == 0
            })
            .collect();
        let mut sequence = String::new();
        while let Some(node) = sorted_queue.pop_first() {
            seen.insert(node);
            sequence.push(node);
            for (_, next, _) in self.instructions.edges_directed(node, Direction::Outgoing) {
                if self
                    .instructions
                    .edges_directed(next, Direction::Incoming)
                    .all(|(before, _, _)| seen.contains(&before))
                {
                    sorted_queue.insert(next);
                }
            }
        }

        sequence
    }
    fn solution_part_2(&self, offset: u32, num_workers: usize) -> u32 {
        let mut seen: HashSet<char> = HashSet::new();
        let mut sorted_queue: BTreeSet<char> = self
            .instructions
            .nodes()
            .filter(|n| {
                self.instructions
                    .edges_directed(*n, Direction::Incoming)
                    .count()
                    == 0
            })
            .collect();
        let mut workers: BTreeSet<(u32, char)> = BTreeSet::new();
        let mut total_ticks = 0;
        while seen.len() < self.instructions.node_count() {
            // allocate steps to workers
            while workers.len() < num_workers && !sorted_queue.is_empty() {
                let current = sorted_queue.pop_first().unwrap();
                let ticks = offset + current as u32 - 'A' as u32 + 1;
                workers.insert((ticks, current));
            }
            // ticks to first finished worker
            let ticks = workers.first().unwrap().0;
            total_ticks += ticks;
            workers = workers.into_iter().map(|(t, c)| (t - ticks, c)).collect();
            // finished jobs
            let mut finished_steps: Vec<char> = Vec::new();
            while let Some(first) = workers.first()
                && first.0 == 0
            {
                let (_, finished_step) = workers.pop_first().unwrap();
                seen.insert(finished_step);
                finished_steps.push(finished_step);
            }
            // check next steps
            for finished_step in finished_steps {
                for (_, next, _) in self
                    .instructions
                    .edges_directed(finished_step, Direction::Outgoing)
                {
                    if self
                        .instructions
                        .edges_directed(next, Direction::Incoming)
                        .all(|(before, _, _)| seen.contains(&before))
                    {
                        sorted_queue.insert(next);
                    }
                }
            }
        }
        total_ticks
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_07.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, "ABGKCMVWYDEHFOPQUILSTNZRJX");

    let result_part2 = challenge.solution_part_2(60, 5);
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 898);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_07_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, "CABDFE");

        let result_part2 = example.solution_part_2(0, 2);
        println!("result day_07 part 2: {result_part2}");
        assert_eq!(result_part2, 15);

        Ok(())
    }
}
