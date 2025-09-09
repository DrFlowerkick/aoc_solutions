//!day_14.rs

use anyhow::Result;
use petgraph::{
    Direction,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
struct Chemical {
    production_quantity: i64,
    storage: i64,
}

impl Default for Chemical {
    fn default() -> Self {
        Chemical {
            production_quantity: 1,
            storage: 0,
        }
    }
}

impl Chemical {
    fn from_str_with_name(value: &str) -> (String, Self) {
        let (production_quantity, name) = value.trim().split_once(' ').unwrap();
        (
            name.to_string(),
            Chemical {
                production_quantity: production_quantity.parse().unwrap(),
                storage: 0,
            },
        )
    }
    fn produce_n_times(&mut self, factor: i64) {
        self.storage += self.production_quantity * factor;
    }
    fn remove_from_storage(&mut self, production_quantity: i64) {
        self.storage -= production_quantity;
    }
    fn refill_storage(&mut self) -> i64 {
        if self.storage >= 0 {
            return 0;
        }
        let factor = self.storage.abs() / self.production_quantity
            + (self.storage.abs() % self.production_quantity > 0) as i64;
        self.produce_n_times(factor);
        factor
    }
}

struct ChallengeInput {
    chemicals: DiGraph<Chemical, i64>,
    ore: NodeIndex,
    fuel: NodeIndex,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut chemicals: DiGraph<Chemical, i64> = DiGraph::new();
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
        for reaction in value.lines() {
            let (inputs, result) = reaction.split_once("=>").unwrap();
            let (name, result) = Chemical::from_str_with_name(result);
            let result_node = *node_map
                .entry(name)
                .and_modify(|rn| {
                    *chemicals.node_weight_mut(*rn).unwrap() = result;
                })
                .or_insert(chemicals.add_node(result));
            for (name, input) in inputs.split(",").map(Chemical::from_str_with_name) {
                let input_node = *node_map
                    .entry(name)
                    .or_insert(chemicals.add_node(Chemical::default()));
                chemicals.add_edge(input_node, result_node, input.production_quantity);
            }
        }
        let ore = *node_map.get(&String::from("ORE")).unwrap();
        let fuel = *node_map.get(&String::from("FUEL")).unwrap();

        ChallengeInput {
            chemicals,
            ore,
            fuel,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> i64 {
        self.generate_fuel(1)
    }
    fn solution_part_2(&mut self) -> i64 {
        let mut low = 0;
        let mut high = 1_000_000_000_000;
        while low < high {
            let mid = (low + high + 1) / 2;
            if self.generate_fuel(mid) < 1_000_000_000_000 {
                // enough ore. can we get more fuel?
                low = mid;
            } else {
                // too much ore. reduce fuel
                high = mid - 1;
            }
        }
        // low is last valid value for ore < 1_000_000_000_000
        low
    }
    fn generate_fuel(&mut self, fuel_production: i64) -> i64 {
        self.reset_storage();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        queue.push_back(self.fuel);
        while let Some(node) = queue.pop_front() {
            if node == self.fuel {
                // we always generate one fuel per recipe. Therefore we can use fuel_production as a factor,
                // if we want to calculate required ore for multiple fuel
                self.chemicals
                    .node_weight_mut(self.fuel)
                    .unwrap()
                    .produce_n_times(fuel_production);
                for (input, required_input) in self.ingredients(node).iter() {
                    queue.push_back(*input);
                    self.chemicals
                        .node_weight_mut(*input)
                        .unwrap()
                        .remove_from_storage(required_input * fuel_production);
                }
            } else {
                let production_factor = self
                    .chemicals
                    .node_weight_mut(node)
                    .unwrap()
                    .refill_storage();
                if production_factor > 0 {
                    for (input, required_input) in self.ingredients(node).iter() {
                        if *input == self.ore {
                            self.chemicals
                                .node_weight_mut(*input)
                                .unwrap()
                                .produce_n_times(production_factor * required_input);
                        } else {
                            queue.push_back(*input);
                            self.chemicals
                                .node_weight_mut(*input)
                                .unwrap()
                                .remove_from_storage(production_factor * required_input);
                        }
                    }
                }
            }
        }
        self.chemicals.node_weight(self.ore).unwrap().storage
    }
    fn reset_storage(&mut self) {
        self.chemicals
            .node_weights_mut()
            .for_each(|c| c.storage = 0);
    }
    fn ingredients(&self, node: NodeIndex) -> Vec<(NodeIndex, i64)> {
        self.chemicals
            .edges_directed(node, Direction::Incoming)
            .map(|e| (e.source(), *e.weight()))
            .collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_14.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, 202_617);

    let result_part2 = challenge.solution_part_2();
    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 7_863_863);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_14() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_14_example.txt");
        let solutions = [
            (31, 0),
            (165, 0),
            (13_312, 82_892_753),
            (180_697, 5_586_022),
            (2_210_736, 460_664),
        ];

        for (input, (solution_1, solution_2)) in multi_input.split("\n\n").zip(solutions) {
            let mut example = ChallengeInput::from(input);

            let result_part1 = example.solution_part_1();
            println!("result day_14 part 1: {result_part1}");
            assert_eq!(result_part1, solution_1);
            if solution_2 != 0 {
                let result_part2 = example.solution_part_2();
                println!("result day_14 part 2: {result_part2}");
                assert_eq!(result_part2, solution_2);
            }
        }

        Ok(())
    }
}
