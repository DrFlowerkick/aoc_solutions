//!day_24.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

struct ChallengeInput {
    map: HashMap<Point, char>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            map: value
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    let y = y as i64;
                    line.char_indices()
                        .map(move |(x, c)| (Point::new(x as i64, y), c))
                        .filter(|(_, c)| *c != '#')
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let distance_map = self.distance_map();
        let digits: HashSet<char> = self
            .map
            .values()
            .filter(|c| c.is_ascii_digit())
            .copied()
            .collect();
        let mut sorted_queue: BTreeSet<(u64, char, Vec<char>)> = BTreeSet::new();
        sorted_queue.insert((0, '0', vec![]));
        let mut min_steps_part_1 = u64::MAX;
        let mut min_steps_part_2 = u64::MAX;
        while let Some((steps, digit, mut visited)) = sorted_queue.pop_first() {
            if steps > min_steps_part_1.max(min_steps_part_2) {
                continue;
            }
            visited.push(digit);
            if visited.len() == digits.len() {
                min_steps_part_1 = min_steps_part_1.min(steps);
                let small = digit.min('0');
                let big = digit.max('0');
                let final_steps = steps + distance_map.get(&(small, big)).unwrap();
                min_steps_part_2 = min_steps_part_2.min(final_steps);
            }
            for next_digit in digits.iter().filter(|d| !visited.contains(d)) {
                let small = digit.min(*next_digit);
                let big = digit.max(*next_digit);
                let next_steps = steps + distance_map.get(&(small, big)).unwrap();
                sorted_queue.insert((next_steps, *next_digit, visited.clone()));
            }
        }
        (min_steps_part_1, min_steps_part_2)
    }
    fn distance_map(&self) -> HashMap<(char, char), u64> {
        let mut distance_map: HashMap<(char, char), u64> = HashMap::new();
        let digits: Vec<(Point, char)> = self
            .map
            .iter()
            .filter(|(_, c)| c.is_ascii_digit())
            .map(|(p, c)| (*p, *c))
            .collect();
        for (i, &(p1, d1)) in digits.iter().enumerate() {
            for &(p2, d2) in digits.iter().skip(i + 1) {
                let distance = self.get_distance(p1, p2);
                let small = d1.min(d2);
                let big = d1.max(d2);
                distance_map.insert((small, big), distance);
            }
        }
        distance_map
    }
    fn get_distance(&self, start: Point, end: Point) -> u64 {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, u64)> = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((pos, steps)) = queue.pop_front() {
            if pos == end {
                return steps;
            }
            if seen.insert(pos) {
                for neighbor in Compass::cardinals()
                    .into_iter()
                    .map(|dir| pos.add(dir))
                    .filter(|n| self.map.contains_key(n))
                {
                    queue.push_back((neighbor, steps + 1));
                }
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_24.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_24 part 1: {result_part1}");
    assert_eq!(result_part1, 456);

    println!("result day_24 part 2: {result_part2}");
    assert_eq!(result_part2, 704);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_24() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_24_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1_and_2().0;
        println!("result day_24 part 1: {result_part1}");
        assert_eq!(result_part1, 14);

        Ok(())
    }
}
