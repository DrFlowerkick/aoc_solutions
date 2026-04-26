//!day_12.rs

use anyhow::Result;
use petgraph::{graphmap::UnGraphMap, visit::Bfs};
use std::collections::HashSet;

struct ChallengeInput<'a> {
    map: UnGraphMap<&'a str, ()>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let mut map = UnGraphMap::new();
        for line in value.lines() {
            let (node, neighbors) = line.split_once(" <-> ").unwrap();
            for neighbor in neighbors.split(", ") {
                map.add_edge(node, neighbor, ());
            }
        }
        ChallengeInput { map }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u64 {
        let mut walker = Bfs::new(&self.map, "0");
        let mut counter = 0;
        while walker.next(&self.map).is_some() {
            counter += 1;
        }
        counter
    }
    fn solution_part_2(&self) -> usize {
        let mut group_start_nodes: HashSet<&str> = HashSet::new();
        let mut seen: HashSet<&str> = HashSet::new();
        while let Some(start) = self.map.nodes().find(|n| !seen.contains(n)) {
            let mut walker = Bfs::new(&self.map, start);
            group_start_nodes.insert(start);
            while let Some(node) = walker.next(&self.map) {
                seen.insert(node);
            }
        }
        group_start_nodes.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 378);

    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 204);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_12() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_12_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_12 part 1: {result_part1}");
        assert_eq!(result_part1, 6);

        let result_part2 = example.solution_part_2();
        println!("result day_12 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
