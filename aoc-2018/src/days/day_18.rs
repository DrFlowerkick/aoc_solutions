//!day_18.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashMap;

struct ChallengeInput {
    map: HashMap<Point, char>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Point::new(x as i64, y as i64);
                map.insert(pos, c);
            }
        }
        ChallengeInput { map }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        for _ in 0..10 {
            self.one_round();
        }
        self.map.values().filter(|v| **v == '|').count()
            * self.map.values().filter(|v| **v == '#').count()
    }
    fn solution_part_2(&mut self) -> usize {
        let mut pattern_round_cache: HashMap<String, usize> = HashMap::new();
        let mut values: Vec<usize> = Vec::new();
        // required to build pattern string
        let positions: Vec<Point> = self.map.keys().copied().collect();
        let max_rounds = 1_000_000_000_usize;
        for round in 1..=max_rounds {
            self.one_round();
            let pattern: String = positions.iter().filter_map(|p| self.map.get(p)).collect();
            let value = self.map.values().filter(|v| **v == '|').count()
                * self.map.values().filter(|v| **v == '#').count();
            values.push(value);
            if let Some(first_seen) = pattern_round_cache.insert(pattern, round) {
                let delta_round = round - first_seen;
                let remaining_rounds = (max_rounds - first_seen) % delta_round;
                // -1 because values start at index 0, rounds start at 1
                let value_index = first_seen + remaining_rounds - 1;
                return values[value_index];
            }
        }
        self.map.values().filter(|v| **v == '|').count()
            * self.map.values().filter(|v| **v == '#').count()
    }
    fn one_round(&mut self) {
        let iter_map = self.map.clone();
        for (pos, acre) in iter_map.iter() {
            let neighbor_iter = Compass::cardinals_and_ordinals()
                .into_iter()
                .map(|c| pos.add(c));
            match acre {
                '.' => {
                    if neighbor_iter
                        .filter_map(|p| iter_map.get(&p))
                        .filter(|a| **a == '|')
                        .count()
                        >= 3
                    {
                        *self.map.get_mut(pos).unwrap() = '|';
                    }
                }
                '|' => {
                    if neighbor_iter
                        .filter_map(|p| iter_map.get(&p))
                        .filter(|a| **a == '#')
                        .count()
                        >= 3
                    {
                        *self.map.get_mut(pos).unwrap() = '#';
                    }
                }
                '#' => {
                    let (is_tree, is_lumber) = neighbor_iter
                        .filter_map(|p| iter_map.get(&p))
                        .fold((false, false), |(t, l), c| (t || *c == '|', l || *c == '#'));
                    if !(is_tree && is_lumber) {
                        *self.map.get_mut(pos).unwrap() = '.';
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_18.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_18 part 1: {result_part1}");
    assert_eq!(result_part1, 623_583);

    // reset map
    let input = include_str!("../../../../aoc_input/aoc-2018/day_18.txt");
    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_18 part 2: {result_part2}");
    assert_eq!(result_part2, 107_912);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_18_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 1_147);

        // no example for part 2

        Ok(())
    }
}
