//!day_13.rs

use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

struct ChallengeInput<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<(&'a str, &'a str), i64>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();
        for line in value.lines() {
            let (first, rem) = line.split_once(" would ").unwrap();
            let (action, rem) = rem.split_once(" ").unwrap();
            let (value, rem) = rem.split_once(" ").unwrap();
            let second = rem
                .strip_prefix("happiness units by sitting next to ")
                .unwrap()
                .strip_suffix(".")
                .unwrap();
            let mut value = value.parse::<i64>().unwrap();
            if action == "lose" {
                value *= -1;
            }
            nodes.insert(first);
            edges.insert((first, second), value);
        }
        ChallengeInput { nodes, edges }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> i64 {
        self.find_max_happiness(self.nodes.clone())
    }
    fn solution_part_2(&self) -> i64 {
        let mut nodes = self.nodes.clone();
        nodes.insert("you");
        self.find_max_happiness(nodes)
    }
    fn find_max_happiness(&self, nodes: HashSet<&str>) -> i64 {
        let mut max_happiness = i64::MIN;
        let mut queue: VecDeque<(&str, Vec<&str>)> =
            nodes.iter().copied().map(|n| (n, vec![])).collect();
        while let Some((node, mut seen)) = queue.pop_front() {
            seen.push(node);
            if seen.len() == nodes.len() {
                let happiness = self.calc_happiness(seen);
                max_happiness = max_happiness.max(happiness);
            } else {
                for next in nodes.difference(&seen.iter().copied().collect()).copied() {
                    queue.push_back((next, seen.clone()));
                }
            }
        }
        max_happiness
    }
    fn calc_happiness(&self, seen: Vec<&str>) -> i64 {
        let mut happiness = 0;
        for (mut index, first) in seen.iter().enumerate() {
            index += 1;
            if index == seen.len() {
                index = 0;
            }
            let second = seen[index];
            happiness += self.edges.get(&(*first, second)).unwrap_or(&0)
                + self.edges.get(&(second, *first)).unwrap_or(&0);
        }
        happiness
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_13.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 709);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, 668);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_13_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, 330);

        Ok(())
    }
}
