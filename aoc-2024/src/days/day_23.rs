//!day_23.rs

use anyhow::Result;
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::GetAdjacencyMatrix,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Day23Data<'a> {
    graph: UnGraph<&'a str, ()>,
}

impl<'a> From<&'a str> for Day23Data<'a> {
    fn from(value: &'a str) -> Self {
        let mut graph: UnGraph<&'a str, ()> = UnGraph::new_undirected();
        let mut map: HashMap<&'a str, NodeIndex> = HashMap::new();
        for (left, right) in value.lines().filter_map(|l| l.split_once('-')) {
            let left = if let Some(ni) = map.get(left) {
                *ni
            } else {
                let ni = graph.add_node(left);
                map.insert(left, ni);
                ni
            };
            let right = if let Some(ni) = map.get(right) {
                *ni
            } else {
                let ni = graph.add_node(right);
                map.insert(right, ni);
                ni
            };
            if graph.find_edge_undirected(left, right).is_none() {
                graph.add_edge(left, right, ());
            }
        }
        Self { graph }
    }
}

impl<'a> Day23Data<'a> {
    fn find_triangles(&self) -> Vec<Vec<NodeIndex>> {
        let mut triangles: Vec<Vec<NodeIndex>> = Vec::new();
        let adj_matrix = self.graph.adjacency_matrix();
        let mut seen: HashSet<Vec<NodeIndex>> = HashSet::new();
        for a in self.graph.node_indices() {
            for (index, b) in self.graph.neighbors(a).enumerate() {
                for c in self
                    .graph
                    .neighbors(a)
                    .skip(index + 1)
                    .filter(|c| self.graph.is_adjacent(&adj_matrix, *c, b))
                {
                    let mut triangle = vec![a, b, c];
                    triangle.sort();
                    if seen.insert(triangle.clone()) && triangle.iter().any(|n| self.is_t(n)) {
                        triangles.push(triangle);
                    }
                }
            }
        }
        triangles
    }
    fn is_t(&self, node: &NodeIndex) -> bool {
        &self.graph[*node][..1] == "t"
    }
    fn find_lan_party(&self) -> String {
        let mut cliques = Vec::new();
        let mut current_clique = HashSet::new();
        let potential_candidates: HashSet<NodeIndex> = self.graph.node_indices().collect();
        let processed_nodes = HashSet::new();
        self.bron_kerbosch(
            &mut current_clique,
            potential_candidates,
            processed_nodes,
            &mut cliques,
        );
        let lan_party = cliques.iter().max_by_key(|c| c.len()).unwrap();
        let mut lan_party: Vec<String> = lan_party
            .iter()
            .map(|n| self.graph[*n].to_string())
            .collect();
        lan_party.sort();
        lan_party.join(",")
    }
    fn bron_kerbosch(
        &self,
        current_clique: &mut HashSet<NodeIndex>,
        mut potential_candidates: HashSet<NodeIndex>,
        mut processed_nodes: HashSet<NodeIndex>,
        cliques: &mut Vec<HashSet<NodeIndex>>,
    ) {
        if potential_candidates.is_empty() && processed_nodes.is_empty() {
            cliques.push(current_clique.clone());
        } else {
            let pc_clone = potential_candidates.clone();
            for ni in pc_clone {
                current_clique.insert(ni);
                let neighbors: HashSet<NodeIndex> = self.graph.neighbors(ni).collect();
                let p_intersection: HashSet<NodeIndex> = potential_candidates
                    .intersection(&neighbors)
                    .cloned()
                    .collect();
                let x_intersection: HashSet<NodeIndex> =
                    processed_nodes.intersection(&neighbors).cloned().collect();
                self.bron_kerbosch(current_clique, p_intersection, x_intersection, cliques);
                current_clique.remove(&ni);
                potential_candidates.remove(&ni);
                processed_nodes.insert(ni);
            }
        }
    }
}

pub fn day_23() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_23.txt");
    let challenge = Day23Data::from(input);

    let result_part1 = challenge.find_triangles().len();
    println!("result day 23 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_075);

    let result_part2 = challenge.find_lan_party();
    println!("result day 23 part 2: {}", result_part2);
    assert_eq!(result_part2, "az,cg,ei,hz,jc,km,kt,mv,sv,sx,wc,wq,xy");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_23_example.txt");
        let challenge = Day23Data::from(input);

        let result_part1 = challenge.find_triangles().len();
        println!("result day 23 part 1: {}", result_part1);
        assert_eq!(result_part1, 7);

        let result_part2 = challenge.find_lan_party();
        println!("result day 23 part 2: {}", result_part2);
        assert_eq!(result_part2, "co,de,ka,ta");

        Ok(())
    }
}
