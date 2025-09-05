//!day_06.rs

use anyhow::Result;
use petgraph::{
    Direction,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use std::collections::{HashMap, HashSet};

struct ChallengeInput {
    orbit_map: DiGraph<String, ()>,
    name_map: HashMap<String, NodeIndex>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut challenge = ChallengeInput {
            orbit_map: DiGraph::new(),
            name_map: HashMap::new(),
        };
        for (center, orbit) in value
            .lines()
            .filter_map(|l| l.split_once(')'))
            .map(|(c, o)| (c.to_string(), o.to_string()))
        {
            let center_node = challenge.add_node(center);
            let orbit_node = challenge.add_node(orbit);
            challenge.orbit_map.add_edge(center_node, orbit_node, ());
        }

        challenge
    }
}

impl ChallengeInput {
    fn add_node(&mut self, object: String) -> NodeIndex {
        if let Some(object_node) = self.name_map.get(&object) {
            *object_node
        } else {
            let object_node = self.orbit_map.add_node(object.clone());
            self.name_map.insert(object, object_node);
            object_node
        }
    }
    fn solution_part_1(&self) -> usize {
        self.orbit_map
            .node_indices()
            .map(|o| self.count_distance_to_com(o).len())
            .sum()
    }
    fn solution_part_2(&self) -> u64 {
        let you_node = *self.name_map.get(&String::from("YOU")).unwrap();
        let you_distance_map = self.count_distance_to_com(you_node);
        let santa_node = *self.name_map.get(&String::from("SAN")).unwrap();
        let santa_distance_map = self.count_distance_to_com(santa_node);
        let you_nodes: HashSet<NodeIndex> = you_distance_map.keys().copied().collect();
        let santa_nodes: HashSet<NodeIndex> = santa_distance_map.keys().copied().collect();
        let intersection: HashSet<NodeIndex> =
            you_nodes.intersection(&santa_nodes).copied().collect();
        let first_common_node = intersection
            .iter()
            .min_by_key(|n| you_distance_map.get(*n).unwrap())
            .unwrap();
        you_distance_map.get(first_common_node).unwrap()
            + santa_distance_map.get(first_common_node).unwrap()
            - 2
    }
    fn count_distance_to_com(&self, mut object: NodeIndex) -> HashMap<NodeIndex, u64> {
        let mut count = 0;
        let mut distance_map: HashMap<NodeIndex, u64> = HashMap::new();
        while let Some(next) = self
            .orbit_map
            .edges_directed(object, Direction::Incoming)
            .next()
        {
            count += 1;
            object = next.source();
            distance_map.insert(object, count);
        }
        distance_map
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_06.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 251_208);

    let result_part2 = challenge.solution_part_2();
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 397);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_06() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_06_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 54);

        let result_part2 = example.solution_part_2();
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 4);

        Ok(())
    }
}
