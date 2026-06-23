//!day_09.rs

use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

struct ChallengeInput<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<(&'a str, &'a str), u64>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();
        for line in value.lines() {
            let (locations, distance) = line.split_once(" = ").unwrap();
            let (left, right) = locations.split_once(" to ").unwrap();
            let min = left.min(right);
            let max = left.max(right);
            nodes.insert(min);
            nodes.insert(max);
            edges.insert((min, max), distance.parse().unwrap());
        }

        ChallengeInput { nodes, edges }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let mut queue: VecDeque<(u64, &str, HashSet<&str>)> =
            self.nodes.iter().map(|n| (0, *n, HashSet::new())).collect();
        let mut min_distance = u64::MAX;
        let mut max_distance = 0;
        while let Some((distance, pos, mut visited)) = queue.pop_front() {
            visited.insert(pos);
            if visited.len() == self.nodes.len() {
                min_distance = min_distance.min(distance);
                max_distance = max_distance.max(distance);
            } else {
                for next in self.nodes.difference(&visited).copied() {
                    let min = pos.min(next);
                    let max = pos.max(next);
                    let next_distance = distance + self.edges.get(&(min, max)).unwrap();
                    queue.push_back((next_distance, next, visited.clone()));
                }
            }
        }
        (min_distance, max_distance)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 117);

    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 909);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_09() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_09_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_09 part 1: {result_part1}");
        assert_eq!(result_part1, 605);

        println!("result day_09 part 2: {result_part2}");
        assert_eq!(result_part2, 982);

        Ok(())
    }
}
