//!day_11.rs

use anyhow::Result;
use petgraph::{
    algo::simple_paths::all_simple_paths,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use std::collections::{HashMap, HashSet, hash_map::RandomState};

struct ChallengeInput<'a> {
    servers: DiGraph<&'a str, ()>,
    node_map: HashMap<&'a str, NodeIndex>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let mut servers = DiGraph::new();
        let mut node_map = HashMap::new();
        for (node, connected_nodes) in value.lines().filter_map(|l| l.split_once(": ")) {
            let node_index = match node_map.get(node) {
                Some(ni) => *ni,
                None => {
                    let ni = servers.add_node(node);
                    node_map.insert(node, ni);
                    ni
                }
            };
            for next_node in connected_nodes.split_whitespace() {
                let next_node_index = match node_map.get(next_node) {
                    Some(ni) => *ni,
                    None => {
                        let ni = servers.add_node(next_node);
                        node_map.insert(next_node, ni);
                        ni
                    }
                };
                servers.add_edge(node_index, next_node_index, ());
            }
        }
        ChallengeInput { servers, node_map }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> usize {
        let you = self.node_map.get("you").unwrap();
        let out = self.node_map.get("out").unwrap();
        all_simple_paths::<HashSet<_>, _, RandomState>(&self.servers, *you, *out, 0, None).count()
    }
    fn solution_part_2(&self) -> usize {
        let svr = *self.node_map.get("svr").unwrap();
        let dac = *self.node_map.get("dac").unwrap();
        let fft = *self.node_map.get("fft").unwrap();
        let out = *self.node_map.get("out").unwrap();
        let routes = [[svr, dac, fft, out], [svr, fft, dac, out]];
        'route_loop: for route in routes {
            let mut count = 1;
            for (index, end) in route[1..].iter().enumerate() {
                count *= self.walk_tree(route[index], *end, &mut HashMap::new());
                if count == 0 {
                    continue 'route_loop;
                }
            }
            return count;
        }
        0
    }
    fn walk_tree(
        &self,
        node: NodeIndex,
        end: NodeIndex,
        seen: &mut HashMap<NodeIndex, usize>,
    ) -> usize {
        if node == end {
            return 1;
        }
        if let Some(seen_node) = seen.get(&node) {
            return *seen_node;
        }
        let count = self
            .servers
            .edges(node)
            .map(|e| self.walk_tree(e.target(), end, seen))
            .sum();

        seen.insert(node, count);
        count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, 500);

    let result_part2 = challenge.solution_part_2();
    println!("result day_11 part 2: {result_part2}");
    assert_eq!(result_part2, 287_039_700_129_600);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_11_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_11 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        let input = include_str!("../../../../aoc_input/aoc-2025/day_11_example_part_2.txt");
        let example = ChallengeInput::from(input);
        let result_part2 = example.solution_part_2();
        println!("result day_11 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
