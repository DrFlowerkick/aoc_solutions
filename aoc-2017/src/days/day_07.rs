//!day_07.rs

use anyhow::Result;
use petgraph::{Direction, graphmap::DiGraphMap};
use std::collections::HashMap;

struct ChallengeInput<'a> {
    weights: HashMap<&'a str, u64>,
    map: DiGraphMap<&'a str, ()>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let mut weights = HashMap::new();
        let mut map = DiGraphMap::new();
        for line in value.lines() {
            let (left, right) = if let Some((left, right)) = line.split_once(" -> ") {
                (left, Some(right))
            } else {
                (line, None)
            };
            // name and value
            let (name, value) = left.split_once(" (").unwrap();
            let value = value.strip_suffix(")").unwrap().parse().unwrap();
            weights.insert(name, value);
            map.add_node(name);
            // child nodes
            if let Some(children) = right {
                for child in children.split(", ") {
                    map.add_edge(name, child, ());
                }
            }
        }

        ChallengeInput { weights, map }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> &str {
        for node in self.map.nodes() {
            if self.map.edges_directed(node, Direction::Incoming).count() == 0 {
                return node;
            }
        }
        ""
    }
    fn solution_part_2(&self, root: &str) -> u64 {
        let mut total_weights = HashMap::new();
        self.collect_weights(root, &mut total_weights);

        for node in self.map.nodes() {
            let children_total_weights: Vec<(&str, u64)> = self
                .map
                .edges_directed(node, Direction::Outgoing)
                .map(|(_, child, _)| (child, total_weights[child]))
                .collect();
            if let Some(min_weight) = children_total_weights.iter().map(|(_, w)| w).min()
                && let Some((name, over_weight)) =
                    children_total_weights.iter().find(|(_, w)| w != min_weight)
            {
                // required weight = current weight - delta
                return self.weights[name] - (over_weight - min_weight);
            }
        }
        0
    }
    fn collect_weights(&'a self, node: &'a str, total_weights: &mut HashMap<&'a str, u64>) -> u64 {
        let mut sum_children = 0;
        for (_, child, _) in self.map.edges_directed(node, Direction::Outgoing) {
            sum_children += self.collect_weights(child, total_weights);
        }
        let total_weight = self.weights[node] + sum_children;
        total_weights.insert(node, total_weight);
        total_weight
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_07.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, "cyrupz");

    let result_part2 = challenge.solution_part_2(result_part1);
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 193);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_07_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, "tknk");

        let result_part2 = example.solution_part_2(result_part1);
        println!("result day_07 part 2: {result_part2}");
        assert_eq!(result_part2, 60);

        Ok(())
    }
}
