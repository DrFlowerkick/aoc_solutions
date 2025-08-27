//!day_07.rs

use anyhow::Result;
use petgraph::{
    Direction,
    graph::{DiGraph, NodeIndex},
    visit::{Bfs, EdgeRef, Walker},
};
use std::collections::HashMap;

struct ChallengeInput {
    bag_tree: DiGraph<String, u64>,
    index_map: HashMap<String, NodeIndex>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut bag_tree = DiGraph::new();
        let mut index_map: HashMap<String, NodeIndex> = HashMap::new();
        for line in value.lines() {
            let (bag, content) = line.split_once("contain").unwrap();
            let bag = bag.trim().strip_suffix("bags").unwrap().trim().to_string();
            let bag_index = *index_map
                .entry(bag.clone())
                .or_insert(bag_tree.add_node(bag));

            match content.trim().strip_suffix('.').unwrap() {
                "no other bags" => (),
                bag_list => bag_list
                    .split(',')
                    .map(|list_item| {
                        let mut word_iter = list_item.split_whitespace();
                        let num_bags: u64 = word_iter.next().unwrap().parse().unwrap();
                        let color = word_iter.fold(String::new(), |color, word| {
                            if color.is_empty() {
                                word.to_string()
                            } else if word == "bag" || word == "bags" {
                                color
                            } else {
                                color + " " + word
                            }
                        });
                        (color, num_bags)
                    })
                    .for_each(|(color, num_bags)| {
                        let color_index = index_map
                            .entry(color.clone())
                            .or_insert(bag_tree.add_node(color));
                        // we point from content to containing bag -> bfs is in direction of directed edge
                        bag_tree.add_edge(*color_index, bag_index, num_bags);
                    }),
            }
        }

        ChallengeInput {
            bag_tree,
            index_map,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let start = *self.index_map.get(&String::from("shiny gold")).unwrap();
        let bfs = Bfs::new(&self.bag_tree, start);
        bfs.iter(&self.bag_tree).skip(1).count()
    }
    fn solution_part_2(&self) -> u64 {
        let start = *self.index_map.get(&String::from("shiny gold")).unwrap();
        self.get_num_of_containing_bags(start)
    }
    fn get_num_of_containing_bags(&self, node: NodeIndex) -> u64 {
        // if no further bags inside, returning 0 is correct
        self.bag_tree
            .edges_directed(node, Direction::Incoming)
            .map(|edge| {
                let num_bags = edge.weight();
                let next_bag = edge.source();
                // num_bags of source bag plus all bags contained in source bag multiplied with num_bags.
                // if source bag does not contain further bags, only num_bags is returned
                num_bags + num_bags * self.get_num_of_containing_bags(next_bag)
            })
            .sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_07.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, 161);

    let result_part2 = challenge.solution_part_2();
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 30_899);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_07_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 4);

        let result_part2 = example.solution_part_2();
        println!("result day_07 part 2: {result_part2}");
        assert_eq!(result_part2, 32);

        Ok(())
    }
}
