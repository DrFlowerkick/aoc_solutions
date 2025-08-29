//!day_16.rs

use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct ValueRange {
    start: u64,
    end: u64,
}

impl From<&str> for ValueRange {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('-').unwrap();
        ValueRange {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

impl ValueRange {
    fn in_range(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }
}

#[derive(Debug)]
struct RangePair {
    one: ValueRange,
    two: ValueRange,
}

impl From<&str> for RangePair {
    fn from(value: &str) -> Self {
        let (one, two) = value.split_once(" or ").unwrap();
        RangePair {
            one: one.into(),
            two: two.into(),
        }
    }
}

impl RangePair {
    fn is_valid(&self, value: u64) -> bool {
        self.one.in_range(value) || self.two.in_range(value)
    }
}

struct ChallengeInput {
    ticket_rules: HashMap<String, RangePair>,
    my_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut section_iter = value.split("\n\n");
        let ticket_rules: HashMap<String, RangePair> = section_iter
            .next()
            .unwrap()
            .lines()
            .filter_map(|l| l.split_once(": "))
            .map(|(name, range)| (name.to_string(), RangePair::from(range)))
            .collect();
        let my_ticket: Vec<u64> = section_iter
            .next()
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();
        let nearby_tickets: Vec<Vec<u64>> = section_iter
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|l| l.split(',').filter_map(|n| n.parse().ok()).collect())
            .collect();
        ChallengeInput {
            ticket_rules,
            my_ticket,
            nearby_tickets,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut sum_invalid = 0;
        for ticket in self.nearby_tickets.iter() {
            for value in ticket.iter() {
                if !self
                    .ticket_rules
                    .values()
                    .any(|range| range.is_valid(*value))
                {
                    sum_invalid += value;
                }
            }
        }
        sum_invalid
    }
    fn solution_part_2(&self) -> u64 {
        self.get_position_names()
            .iter()
            .zip(self.my_ticket.iter())
            .filter(|(name, _)| name.strip_prefix("departure").is_some())
            .map(|(_, val)| *val)
            .product()
    }
    fn get_position_names(&self) -> Vec<String> {
        // first collect possible names, which fit in every nearby ticket a given ticket value
        let mut possible_position_names: Vec<HashSet<String>> =
            vec![HashSet::new(); self.my_ticket.len()];
        for ticket in self.nearby_tickets.iter().filter(|values| {
            values
                .iter()
                .all(|v| self.ticket_rules.values().any(|range| range.is_valid(*v)))
        }) {
            let mut ticket_possible_position_names: Vec<HashSet<String>> =
                vec![HashSet::new(); self.my_ticket.len()];
            for (index, value) in ticket.iter().enumerate() {
                for (name, _) in self
                    .ticket_rules
                    .iter()
                    .filter(|(_, range)| range.is_valid(*value))
                {
                    ticket_possible_position_names[index].insert(name.to_owned());
                }
            }
            possible_position_names = possible_position_names
                .iter()
                .zip(ticket_possible_position_names.iter())
                .map(|(ppn, tppn)| {
                    if ppn.is_empty() {
                        tppn.clone()
                    } else {
                        ppn.intersection(tppn).cloned().collect()
                    }
                })
                .collect();
        }
        // reduce possible names to solution
        let mut position_names: Vec<String> = vec!["".into(); self.my_ticket.len()];
        while position_names.iter().any(|n| n.is_empty()) {
            let (single_position, name) = possible_position_names
                .iter()
                .enumerate()
                .filter(|(_, ppn)| ppn.len() == 1)
                .map(|(i, ppn)| (i, ppn.iter().next().unwrap()))
                .next()
                .expect("could not identify single name");
            position_names[single_position] = name.to_owned();
            // remove name from HashSets
            let name = name.to_owned();
            possible_position_names.iter_mut().for_each(|ppn| {
                ppn.remove(&name);
            });
        }
        position_names
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_16.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, 23_009);

    let result_part2 = challenge.solution_part_2();
    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, 10_458_887_314_153);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_16() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_16_example_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_16 part 1: {result_part1}");
        assert_eq!(result_part1, 71);

        let input = include_str!("../../../../aoc_input/aoc-2020/day_16_example_2.txt");
        let example = ChallengeInput::from(input);

        let position_names = example.get_position_names();
        println!("day_16 part 2 position names: {:?}", position_names);
        assert_eq!(position_names, ["row", "class", "seat"]);

        Ok(())
    }
}
