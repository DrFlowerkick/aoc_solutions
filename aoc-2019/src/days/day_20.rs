//!day_20.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use petgraph::{
    algo::dijkstra,
    graph::{NodeIndex, UnGraph},
};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

struct ChallengeInput<const X: usize, const Y: usize> {
    map: MyMap2D<char, X, Y>,
    graph: UnGraph<MapPoint<X, Y>, usize>,
    node_map: HashMap<MapPoint<X, Y>, NodeIndex>,
    portal_map: HashMap<NodeIndex, (NodeIndex, i16)>,
    aa: NodeIndex,
    zz: NodeIndex,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        ChallengeInput {
            map: MyMap2D::from(value),
            graph: UnGraph::new_undirected(),
            node_map: HashMap::new(),
            portal_map: HashMap::new(),
            aa: NodeIndex::default(),
            zz: NodeIndex::default(),
        }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&mut self) -> usize {
        self.analyze_map_part_1();
        let distance_map = dijkstra(&self.graph, self.aa, None, |e| *e.weight());
        *distance_map.get(&self.zz).unwrap()
    }
    fn solution_part_2(&mut self) -> usize {
        self.analyze_map_part_2();
        let mut seen: HashMap<(NodeIndex, i16), usize> = HashMap::new();
        seen.insert((self.aa, 0), 0);
        // insert all outer nodes at level 0 to seen, since these portals are walls
        for (outer, _) in self.portal_map.iter().filter(|(_, (_, l))| *l < 0) {
            seen.insert((*outer, 0), 0);
        }
        let mut queue: BTreeSet<LevelWalker> = BTreeSet::new();
        queue.insert(LevelWalker {
            steps: 0,
            level: 0,
            node: self.aa,
        });
        while let Some(walker) = queue.pop_first() {
            if walker.level == 0 && walker.node == self.zz {
                // reached end of maze
                return walker.steps;
            }
            let distance_map = dijkstra(&self.graph, walker.node, None, |e| *e.weight());
            for (node, distance) in distance_map.iter() {
                if let Some(seen_dist) = seen.get(&(*node, walker.level))
                    && walker.steps + distance >= *seen_dist
                {
                    // we ignore all already seen nodes at given level, if seen_dist
                    // is smaller than traveled steps
                    continue;
                }
                // insert or update seen with traveled steps to node at this level
                seen.insert((*node, walker.level), walker.steps + distance);
                if walker.level > 0 && (self.aa == *node || self.zz == *node) {
                    // ignore entry / exit node at deeper levels
                    continue;
                }
                // reached exit -> queue it
                if walker.level == 0 && self.zz == *node {
                    let mut next = walker;
                    next.steps += distance;
                    next.node = *node;
                    queue.insert(next);
                    continue;
                }
                // reached portal -> move to other side (1 step) and queue it
                let (portal, level) = self.portal_map.get(node).unwrap();
                let mut next = walker;
                next.steps += distance + 1;
                next.node = *portal;
                next.level += level;
                seen.insert((next.node, next.level), next.steps);
                queue.insert(next);
            }
        }

        0
    }
    fn analyze_map_part_1(&mut self) {
        // collect portals
        let portals = self.prepare_analyze_map();

        // add portals to graph
        for (p1, c1) in portals.iter() {
            for (p2, _) in portals.iter().filter(|(p2, c2)| c1 == *c2 && p1 != *p2) {
                let p1 = self.get_portal_entry_and_level_change(p1.0, p1.1).0;
                let p1_node = self.graph.add_node(p1);
                self.node_map.insert(p1, p1_node);
                let p2 = self.get_portal_entry_and_level_change(p2.0, p2.1).0;
                let p2_node = self.graph.add_node(p2);
                self.node_map.insert(p2, p2_node);
                self.graph.add_edge(p1_node, p2_node, 1);
            }
        }
        // get distance between each portal, which can be reached via maze
        self.add_distances_to_graph();
    }
    fn analyze_map_part_2(&mut self) {
        // collect portals
        let portals = self.prepare_analyze_map();

        // add portals to graph
        for (p1, c1) in portals.iter() {
            for (p2, _) in portals.iter().filter(|(p2, c2)| c1 == *c2 && p1 != *p2) {
                let (p1, l1) = self.get_portal_entry_and_level_change(p1.0, p1.1);
                let p1_node = self.graph.add_node(p1);
                self.node_map.insert(p1, p1_node);
                let (p2, l2) = self.get_portal_entry_and_level_change(p2.0, p2.1);
                let p2_node = self.graph.add_node(p2);
                self.node_map.insert(p2, p2_node);
                // THIS is the only difference to part 1
                // instead of adding an edge, we link nodes via portal_map
                // we add information of level change. inner portals increase level, outer portals decrease level
                self.portal_map.insert(p1_node, (p2_node, l1));
                self.portal_map.insert(p2_node, (p1_node, l2));
            }
        }
        // get distance between each portal, which can be reached via maze
        self.add_distances_to_graph();
    }
    fn prepare_analyze_map(&mut self) -> HashMap<(MapPoint<X, Y>, MapPoint<X, Y>), (char, char)> {
        // reset graph and maps
        self.graph.clear();
        self.node_map.clear();
        self.portal_map.clear();
        // collect portals
        let mut portals: HashMap<(MapPoint<X, Y>, MapPoint<X, Y>), (char, char)> = HashMap::new();
        // analyze columns
        for ((p1, c1), (p2, c2)) in (0..X)
            .flat_map(|c| self.map.iter_column(c).zip(self.map.iter_column(c).skip(1)))
            .filter(|((_, c1), (_, c2))| c1.is_ascii_uppercase() && c2.is_ascii_uppercase())
        {
            portals.insert((p1, p2), (*c1, *c2));
        }
        // analyze rows
        for ((p1, c1), (p2, c2)) in (0..Y)
            .flat_map(|r| self.map.iter_row(r).zip(self.map.iter_row(r).skip(1)))
            .filter(|((_, c1), (_, c2))| c1.is_ascii_uppercase() && c2.is_ascii_uppercase())
        {
            portals.insert((p1, p2), (*c1, *c2));
        }
        // AA
        let ((p1, p2), _) = portals
            .iter()
            .find(|(_, (c1, c2))| *c1 == 'A' && *c2 == 'A')
            .unwrap();
        let aa = self.get_portal_entry_and_level_change(*p1, *p2).0;
        self.aa = self.graph.add_node(aa);
        self.node_map.insert(aa, self.aa);
        // ZZ
        let ((p1, p2), _) = portals
            .iter()
            .find(|(_, (c1, c2))| *c1 == 'Z' && *c2 == 'Z')
            .unwrap();
        let zz = self.get_portal_entry_and_level_change(*p1, *p2).0;
        self.zz = self.graph.add_node(zz);
        self.node_map.insert(zz, self.zz);
        portals
    }
    fn add_distances_to_graph(&mut self) {
        for (portal, portal_node) in self.node_map.iter() {
            let filter_fn = Box::new(
                |_point_of_next_cell: MapPoint<X, Y>,
                 value_of_next_cell: &char,
                 _orientation_of_next_cell: Compass,
                 _current_point: MapPoint<X, Y>,
                 _value_of_current_cell: &char,
                 _current_distance: usize| { *value_of_next_cell == '.' },
            );
            for (point, _, distance) in self.map.iter_distance(*portal, filter_fn) {
                if distance > 0 && self.node_map.contains_key(&point) {
                    // connect portals
                    let node = self.node_map.get(&point).unwrap();
                    self.graph.add_edge(*portal_node, *node, distance);
                }
            }
        }
    }

    fn get_portal_entry_and_level_change(
        &self,
        p1: MapPoint<X, Y>,
        p2: MapPoint<X, Y>,
    ) -> (MapPoint<X, Y>, i16) {
        let (entry, direction) = [p1, p2]
            .iter()
            .flat_map(|p| p.iter_neighbors(Compass::N, false, false, false))
            .find(|(p, _)| *self.map.get(*p) == '.')
            .unwrap();
        let level: bool = match direction {
            Compass::N => entry.offset_pp((0, 3)).is_some(),
            Compass::S => entry.offset_mm((0, 3)).is_some(),
            Compass::E => entry.offset_mm((3, 0)).is_some(),
            Compass::W => entry.offset_pp((3, 0)).is_some(),
            _ => unreachable!(),
        };
        let level = if level { 1 } else { -1 };
        (entry, level)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LevelWalker {
    steps: usize,
    level: i16,
    node: NodeIndex,
}

impl PartialOrd for LevelWalker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LevelWalker {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.level.cmp(&other.level) {
            Ordering::Equal => match self.steps.cmp(&other.steps) {
                Ordering::Equal => self.node.cmp(&other.node),
                ord => ord,
            },
            ord => ord,
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_20.txt");
    let mut challenge = ChallengeInput::<107, 109>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_20 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_20 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_20_example_1.txt");
        let mut example = ChallengeInput::<21, 19>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 23);

        let result_part2 = example.solution_part_2();
        println!("result day_20 part 2: {result_part2}");
        assert_eq!(result_part2, 26);

        Ok(())
    }

    #[test]
    fn test_example_2_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_20_example_2.txt");
        let mut example = ChallengeInput::<35, 37>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 58);

        Ok(())
    }

    #[test]
    fn test_example_3_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_20_example_3.txt");
        let mut example = ChallengeInput::<45, 37>::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_20 part 2: {result_part2}");
        assert_eq!(result_part2, 396);

        Ok(())
    }
}
