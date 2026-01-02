//!day_20.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use petgraph::graphmap::UnGraphMap;
use std::str::Chars;

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(input: &'a str) -> Self {
        ChallengeInput { input }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1_and_2(&mut self) -> (u64, usize) {
        let start = Point::default();
        let mut graph: UnGraphMap<Point, u64> = UnGraphMap::new();
        let mut direction_iter = self.input.chars();
        assert_eq!(direction_iter.next(), Some('^'));
        self.build_graph(start, 0, &mut direction_iter, &mut graph);
        (
            *graph.all_edges().map(|e| e.2).max().unwrap(),
            graph.all_edges().filter(|(_, _, e)| **e >= 1_000).count(),
        )
    }
    #[allow(clippy::only_used_in_recursion)]
    fn build_graph(
        &self,
        start: Point,
        start_distance: u64,
        direction_iter: &mut Chars,
        graph: &mut UnGraphMap<Point, u64>,
    ) {
        let mut node = start;
        let mut distance = start_distance;
        loop {
            let dir = direction_iter.next().unwrap();
            let new_node = match dir {
                '$' | ')' => return,
                '|' => {
                    node = start;
                    distance = start_distance;
                    continue;
                }
                '(' => {
                    self.build_graph(node, distance, direction_iter, graph);
                    continue;
                }
                'N' => node.add(Compass::N),
                'E' => node.add(Compass::E),
                'S' => node.add(Compass::S),
                'W' => node.add(Compass::W),
                _ => unreachable!(),
            };
            if graph.contains_edge(node, new_node) {
                distance -= 1;
            } else {
                distance += 1;
                graph.add_edge(node, new_node, distance);
            }
            node = new_node;
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_20.txt");
    let mut challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_20 part 1: {result_part1}");
    assert_eq!(result_part1, 3_469);

    println!("result day_20 part 2: {result_part2}");
    assert_eq!(result_part2, 8_780);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_20_example.txt");

        let solutions = [3, 10, 18, 23, 31];

        for (line, solution_part_1) in input.lines().zip(solutions) {
            let mut example = ChallengeInput::from(line);

            let (result_part1, _) = example.solution_part_1_and_2();
            println!("result day_20 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part_1);
        }

        // no example for part 2

        Ok(())
    }
}
