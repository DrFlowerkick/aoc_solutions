//!day_12.rs

use anyhow::Result;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet, hash_map::Entry};

struct Cave {
    size: bool,
}

struct ChallengeInput {
    caves: UnGraph<Cave, ()>,
    start_id: NodeIndex,
    end_id: NodeIndex,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut caves: UnGraph<Cave, ()> = UnGraph::new_undirected();
        let mut cache: HashMap<&str, NodeIndex> = HashMap::new();
        let mut start_id: Option<NodeIndex> = None;
        let mut end_id: Option<NodeIndex> = None;
        for line in value.lines() {
            let indices: Vec<NodeIndex> = line
                .trim()
                .split('-')
                .map(|node_name| match cache.entry(node_name) {
                    Entry::Vacant(entry) => {
                        let size = node_name == "start"
                            || node_name == "end"
                            || node_name.chars().all(|c| c.is_uppercase());
                        let node = caves.add_node(Cave { size });
                        if node_name == "start" {
                            start_id = Some(node);
                        }
                        if node_name == "end" {
                            end_id = Some(node);
                        }
                        *entry.insert(node)
                    }
                    Entry::Occupied(entry) => *entry.get(),
                })
                .collect();
            assert_eq!(indices.len(), 2);
            caves.add_edge(indices[0], indices[1], ());
        }
        ChallengeInput {
            caves,
            start_id: start_id.unwrap(),
            end_id: end_id.unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut seen: HashSet<NodeIndex> = HashSet::new();
        seen.insert(self.start_id);
        self.count_path(self.start_id, seen)
    }
    fn count_path(&self, node: NodeIndex, mut seen: HashSet<NodeIndex>) -> u64 {
        if node == self.end_id {
            return 1;
        }
        if !self.caves[node].size {
            seen.insert(node);
        }
        let mut num_path = 0;
        for next_node in self.caves.neighbors(node) {
            if !seen.contains(&next_node) {
                num_path += self.count_path(next_node, seen.clone());
            }
        }
        num_path
    }
    fn solution_part_2(&self) -> u64 {
        self.count_path_single_twice(self.start_id, HashMap::new())
    }
    fn count_path_single_twice(&self, node: NodeIndex, mut seen: HashMap<NodeIndex, u8>) -> u64 {
        if node == self.end_id {
            return 1;
        }
        let mut twice_visited = seen.values().any(|v| *v == 2);
        if !self.caves[node].size {
            match seen.entry(node) {
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                    twice_visited = true;
                }
            }
        }
        let mut num_path = 0;
        for next_node in self.caves.neighbors(node).filter(|n| *n != self.start_id) {
            if seen.keys().all(|k| *k != next_node) || !twice_visited {
                num_path += self.count_path_single_twice(next_node, seen.clone());
            }
        }
        num_path
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 4_495);

    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 131_254);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_12_example_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1_1 = example.solution_part_1();
        println!("result day_12 part 1 example 1: {result_part1_1}");
        assert_eq!(result_part1_1, 10);

        let result_part2_1 = example.solution_part_2();
        println!("result day_12 part 2 example 1: {result_part2_1}");
        assert_eq!(result_part2_1, 36);

        let input = include_str!("../../../../aoc_input/aoc-2021/day_12_example_2.txt");
        let example = ChallengeInput::from(input);

        let result_part1_2 = example.solution_part_1();
        println!("result day_12 part 1 example 2: {result_part1_2}");
        assert_eq!(result_part1_2, 19);

        let result_part2_2 = example.solution_part_2();
        println!("result day_12 part 2 example 2: {result_part2_2}");
        assert_eq!(result_part2_2, 103);

        let input = include_str!("../../../../aoc_input/aoc-2021/day_12_example_3.txt");
        let example = ChallengeInput::from(input);

        let result_part1_3 = example.solution_part_1();
        println!("result day_12 part 1 example 3: {result_part1_3}");
        assert_eq!(result_part1_3, 226);

        let result_part2_3 = example.solution_part_2();
        println!("result day_12 part 2 example 3: {result_part2_3}");
        assert_eq!(result_part2_3, 3_509);

        Ok(())
    }
}
