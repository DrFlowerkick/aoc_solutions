//!day_23.rs
// ############################################################################
// # INFO: this solution took about 21 seconds for part 2 on my notebook      #
// ############################################################################

use anyhow::Result;
use petgraph::{Direction, graphmap::DiGraphMap};
use std::fmt::Write;
use std::time::Instant;

struct ChallengeInput {
    cup_map: DiGraphMap<usize, ()>,
    current: usize,
    min_val: usize,
    max_val: usize,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut cup_map: DiGraphMap<usize, ()> = DiGraphMap::new();
        // add nodes to graph
        let mut current = 0;
        let mut previous = 0;
        let mut min_val = usize::MAX;
        let mut max_val = 0;
        value
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as usize)
            .for_each(|cup| {
                cup_map.add_node(cup);
                min_val = min_val.min(cup);
                max_val = max_val.max(cup);
                if current == 0 {
                    current = cup;
                } else {
                    cup_map.add_edge(previous, cup, ());
                }
                previous = cup;
            });
        // close ring (previous is now last element in ring)
        cup_map.add_edge(previous, current, ());

        ChallengeInput {
            cup_map,
            current,
            min_val,
            max_val,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> String {
        self.do_moves(100);

        let mut cups = String::new();
        let mut cup = 1;
        loop {
            let next = self.next(cup);
            if next == 1 {
                break;
            }
            cup = next;
            write!(&mut cups, "{cup}").unwrap();
        }
        cups
    }
    fn solution_part_2(&mut self) -> usize {
        let start = Instant::now();
        self.extend_to_val(1_000_000);
        self.do_moves(10_000_000);

        println!("time of execution: {:?}", start.elapsed());
        let after_1 = self.next(1);
        let after_after_1 = self.next(after_1);
        after_1 * after_after_1
    }
    fn do_moves(&mut self, moves: usize) {
        for _ in 0..moves {
            let (pick_up, end_of_pick_up, destination) = self.get_move_cups();
            let after_end_of_pick_up = self.next(end_of_pick_up);
            let after_destination = self.next(destination);
            // remove edges
            for cup in [self.current, end_of_pick_up, destination] {
                let next = self.next(cup);
                self.cup_map.remove_edge(cup, next);
            }
            // add edges
            for (cup, next) in [
                (self.current, after_end_of_pick_up),
                (destination, pick_up),
                (end_of_pick_up, after_destination),
            ] {
                self.cup_map.add_edge(cup, next, ());
            }
            // set current to next
            self.current = self.next(self.current);
        }
    }
    fn next(&self, cup: usize) -> usize {
        self.cup_map
            .edges_directed(cup, Direction::Outgoing)
            .map(|e| e.1)
            .next()
            .unwrap()
    }
    fn get_move_cups(&self) -> (usize, usize, usize) {
        let pick_up = self.next(self.current);
        let second_pick_up = self.next(pick_up);
        let end_of_pick_up = self.next(second_pick_up);
        let mut destination = self.current;
        loop {
            destination = if destination == self.min_val {
                self.max_val
            } else {
                destination - 1
            };
            if destination != pick_up
                && destination != second_pick_up
                && destination != end_of_pick_up
            {
                break;
            }
        }
        (pick_up, end_of_pick_up, destination)
    }
    fn extend_to_val(&mut self, new_max: usize) {
        if new_max <= self.max_val {
            return;
        }
        // get "last" element -> element which points to current
        let mut cup = self.current;
        loop {
            let next = self.next(cup);
            if next == self.current {
                break;
            }
            cup = next;
        }
        // remove "wrap around" edge from last to first element
        self.cup_map.remove_edge(cup, self.current);
        // extend map
        for val in self.max_val + 1..=new_max {
            self.cup_map.add_node(val);
            self.cup_map.add_edge(cup, val, ());
            cup = val;
        }
        // close ring again
        self.cup_map.add_edge(new_max, self.current, ());
        self.max_val = new_max;
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_23.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, "65432978");

    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 287_230_227_046);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_23_petgraph() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_23_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1: {result_part1}");
        assert_eq!(result_part1, "67384529");

        let input = include_str!("../../../../aoc_input/aoc-2020/day_23_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        assert_eq!(result_part2, 149_245_887_792);

        Ok(())
    }
}
