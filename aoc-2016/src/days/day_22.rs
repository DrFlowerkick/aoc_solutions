//!day_22.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    size: u64,
    used: u64,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut node_iter = value
            .split_whitespace()
            .filter_map(|v| v[..v.len() - 1].parse().ok());
        Node {
            size: node_iter.next().unwrap(),
            used: node_iter.next().unwrap(),
        }
    }
}

impl Node {
    fn check_space(&self, other: &Self) -> bool {
        self.size - self.used >= other.used
    }
    fn is_empty(&self) -> bool {
        self.used == 0
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    map: Vec<Vec<Node>>,
    pos: Point,
    hole: Point,
}

impl From<&ChallengeInput> for State {
    fn from(value: &ChallengeInput) -> Self {
        State {
            map: (0..=value.max_y)
                .map(|y| {
                    (0..=value.max_x)
                        .filter_map(|x| value.nodes.get(&(x, y).into()))
                        .copied()
                        .collect()
                })
                .collect(),
            pos: Point::new(value.max_x, 0),
            hole: *value.nodes.iter().find(|(_, n)| n.is_empty()).unwrap().0,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.delta_pos().cmp(&other.delta_pos()) {
            Ordering::Equal => match self.delta_hole().cmp(&other.delta_hole()) {
                Ordering::Equal => match self.delta_pos_hole().cmp(&other.delta_pos_hole()) {
                    Ordering::Equal => self.map.cmp(&other.map),
                    cmp => cmp,
                },
                cmp => cmp,
            },
            cmp => cmp,
        }
    }
}

impl State {
    fn get(&self, pos: &Point) -> Node {
        self.map[pos.y as usize][pos.x as usize]
    }
    fn move_node_data(&mut self, source: Point, target: Point) {
        self.map[target.y as usize][target.x as usize].used +=
            self.map[source.y as usize][source.x as usize].used;
        self.map[source.y as usize][source.x as usize].used = 0;
        if source == self.pos {
            self.pos = target;
        }
        if target == self.hole {
            // this should be always the case
            self.hole = source;
        }
    }
    fn delta_pos(&self) -> i64 {
        self.pos.delta(Point::default())
    }
    fn delta_hole(&self) -> i64 {
        self.hole.delta(Point::default())
    }
    fn delta_pos_hole(&self) -> i64 {
        self.pos.delta(self.hole)
    }
}

#[derive(Clone)]
struct ChallengeInput {
    nodes: HashMap<Point, Node>,
    max_x: i64,
    max_y: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let nodes: HashMap<Point, Node> = value
            .lines()
            .skip(2)
            .map(|l| {
                let (pos, node) = l.split_once(" ").unwrap();
                let pos = pos.strip_prefix("/dev/grid/node-x").unwrap();
                let (x, y) = pos.split_once("-y").unwrap();
                let pos = Point::new(x.parse().unwrap(), y.parse().unwrap());
                let node = Node::from(node);
                (pos, node)
            })
            .collect();
        ChallengeInput {
            max_x: nodes.keys().map(|p| p.x).max().unwrap(),
            max_y: nodes.keys().map(|p| p.y).max().unwrap(),
            nodes,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let positions: Vec<Point> = self.nodes.keys().copied().collect();
        let mut count = 0;
        for (i, node_a) in positions
            .iter()
            .filter_map(|p| self.nodes.get(p))
            .enumerate()
        {
            for node_b in positions
                .iter()
                .skip(i + 1)
                .filter_map(|p| self.nodes.get(p))
            {
                if (!node_a.is_empty() && node_b.check_space(node_a))
                    || (!node_b.is_empty() && node_a.check_space(node_b))
                {
                    count += 1;
                }
            }
        }
        count
    }
    fn solution_part_2(&self) -> u64 {
        // 1. check that there is only one "hole"
        assert_eq!(self.nodes.values().filter(|n| n.is_empty()).count(), 1);

        // 2. check that the only way to move data is to move the data to the hole
        // and that always only one node can move his data to the hole.
        let values: Vec<Node> = self.nodes.values().copied().collect();
        for (i, node_a) in values.iter().enumerate() {
            if node_a.is_empty() {
                continue;
            }
            for node_b in values.iter().skip(i + 1) {
                if node_b.is_empty() {
                    continue;
                }
                // if not hole, than space should always be to small
                assert!(!node_a.check_space(node_b));
                assert!(!node_b.check_space(node_a));
            }
        }

        // 3. check that all nodes in first row do fit in hole
        let hole = self.nodes.values().find(|n| n.is_empty()).unwrap();
        let initial_state = State::from(self);
        for node in initial_state.map[0].iter() {
            // all nodes of first line should fit in hole.
            assert!(hole.check_space(node));
        }

        // Solution:
        // 1. move hole on shortest path to the left of the top left data node
        let (left_state, left_steps) = self.move_hole_to_left_of_top_left(initial_state);

        // 2. move data node from right to left
        // seen is mostly used to prevent hole from moving backward
        let mut seen: HashSet<State> = HashSet::new();
        let mut sorted_queue: BTreeSet<(State, u64)> = BTreeSet::new();
        sorted_queue.insert((left_state, left_steps));
        while let Some((state, steps)) = sorted_queue.pop_first() {
            if seen.insert(state.clone()) {
                if state.pos == Point::default() {
                    return steps;
                }
                let hole = state.hole;
                let hole_node = state.get(&hole);
                for next_pos in Compass::cardinals()
                    .into_iter()
                    .map(|dir| hole.add(dir))
                    .filter(|neighbor| {
                        self.nodes.contains_key(neighbor)
                            && hole_node.check_space(&state.get(neighbor))
                    })
                {
                    let mut next_state = state.clone();
                    next_state.move_node_data(next_pos, hole);
                    sorted_queue.insert((next_state, steps + 1));
                }
            }
        }
        0
    }
    fn move_hole_to_left_of_top_left(&self, initial_state: State) -> (State, u64) {
        let left_of_top_left = Point::new(self.max_x - 1, 0);
        let mut queue: VecDeque<(State, u64)> = VecDeque::new();
        let mut seen: HashSet<Point> = HashSet::new();
        queue.push_back((initial_state.clone(), 0));
        while let Some((state, steps)) = queue.pop_front() {
            if state.hole == left_of_top_left {
                return (state, steps);
            }
            if seen.insert(state.hole) {
                let hole = state.hole;
                let hole_node = state.get(&hole);
                for next_pos in Compass::cardinals()
                    .into_iter()
                    .map(|dir| hole.add(dir))
                    .filter(|neighbor| {
                        self.nodes.contains_key(neighbor)
                            && hole_node.check_space(&state.get(neighbor))
                    })
                {
                    let mut next_state = state.clone();
                    next_state.move_node_data(next_pos, hole);
                    queue.push_back((next_state, steps + 1));
                }
            }
        }
        (initial_state, 0)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_22.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 910);

    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 222);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_22_example.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_22 part 2: {result_part2}");
        assert_eq!(result_part2, 7);

        Ok(())
    }
}
