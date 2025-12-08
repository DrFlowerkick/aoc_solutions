//!day_08.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point3D;
use petgraph::{graphmap::UnGraphMap, visit::Dfs};
use std::collections::HashSet;

struct ChallengeInput {
    circuits: UnGraphMap<Point3D, ()>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut circuits: UnGraphMap<Point3D, ()> = UnGraphMap::new();
        for line in value.lines() {
            let coordinates: Vec<i64> = line.split(',').map(|c| c.parse().unwrap()).collect();
            let point = Point3D::new(coordinates[0], coordinates[1], coordinates[2]);
            circuits.add_node(point);
        }
        ChallengeInput { circuits }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&mut self, num_shortest_connections: usize) -> (usize, i64) {
        let node_count = self.circuits.node_count();
        let mut distances: Vec<(f64, Point3D, Point3D)> =
            Vec::with_capacity(node_count * node_count);
        for (i, a) in self.circuits.nodes().enumerate() {
            for b in self.circuits.nodes().skip(i + 1) {
                distances.push((a.subtract(b).length(), a, b));
            }
        }
        let mut connected_junctions: HashSet<Point3D> =
            HashSet::with_capacity(num_shortest_connections * 2);
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));
        for (_d, a, b) in distances.iter().take(num_shortest_connections) {
            self.circuits.add_edge(*a, *b, ());
            connected_junctions.insert(*a);
            connected_junctions.insert(*b);
        }
        let mut seen: HashSet<Point3D> = HashSet::new();
        let mut circuit_sizes: Vec<usize> = Vec::with_capacity(num_shortest_connections);
        for junction in connected_junctions {
            if seen.contains(&junction) {
                continue;
            }
            let mut dfs = Dfs::new(&self.circuits, junction);
            let mut circuit_size = 0;
            while let Some(nx) = dfs.next(&self.circuits) {
                seen.insert(nx);
                circuit_size += 1;
            }
            circuit_sizes.push(circuit_size);
        }
        circuit_sizes.sort();
        circuit_sizes.reverse();
        let solution_part_1 = circuit_sizes.into_iter().take(3).product();

        // part 2
        for (_d, a, b) in distances.into_iter().skip(num_shortest_connections) {
            self.circuits.add_edge(a, b, ());
            let mut dfs = Dfs::new(&self.circuits, a);
            let mut circuit_size = 0;
            while dfs.next(&self.circuits).is_some() {
                circuit_size += 1;
            }
            if circuit_size == node_count {
                return (solution_part_1, a.x * b.x);
            }
        }

        (solution_part_1, 0)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_08.txt");
    let mut challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(1_000);
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 127_551);

    println!("result day_08 part 2: {result_part2}");
    assert_eq!(result_part2, 2_347_225_200);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_08_example.txt");
        let mut example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2(10);
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 40);

        println!("result day_08 part 2: {result_part2}");
        assert_eq!(result_part2, 25_272);

        Ok(())
    }
}
