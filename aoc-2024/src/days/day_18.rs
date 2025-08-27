//!day_18.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint};
use std::collections::{hash_map::Entry, HashMap, VecDeque};

#[derive(Debug)]
struct Day18Data<const N: usize> {
    bytes: Vec<MapPoint<N, N>>,
    start: MapPoint<N, N>,
    end: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day18Data<N> {
    fn from(value: &str) -> Self {
        let bytes: Vec<MapPoint<N, N>> = value
            .lines()
            .filter_map(|l| l.split_once(','))
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()).into())
            .collect();
        Self {
            bytes,
            start: (0, 0).into(),
            end: (N - 1, N - 1).into(),
        }
    }
}

impl<const N: usize> Day18Data<N> {
    fn shortest_path(&self, n_bytes: usize) -> Option<(usize, HashMap<MapPoint<N, N>, usize>)> {
        let mut min_steps: Option<usize> = None;
        let mut seen: HashMap<MapPoint<N, N>, usize> = self.bytes[..n_bytes]
            .iter()
            .map(|b| (*b, usize::MAX))
            .collect();
        let mut visit: VecDeque<(MapPoint<N, N>, usize)> = VecDeque::new();
        seen.insert(self.start, 0);
        visit.push_back((self.start, 0));
        while let Some((byte, steps)) = visit.pop_front() {
            if byte == self.end {
                min_steps = Some(steps);
                break;
            }
            for neighbor in [Compass::N, Compass::E, Compass::S, Compass::W]
                .iter()
                .filter_map(|d| byte.neighbor(*d))
            {
                if let Entry::Vacant(e) = seen.entry(neighbor) {
                    e.insert(steps + 1);
                    visit.push_back((neighbor, steps + 1));
                }
            }
        }
        min_steps.map(|ms| (ms, seen))
    }
    fn first_block(&self, mut n_bytes: usize) -> MapPoint<N, N> {
        let mut best_path: HashMap<MapPoint<N, N>, usize> = HashMap::new();
        while n_bytes < self.bytes.len() {
            if best_path.is_empty() {
                if let Some((ms, seen)) = self.shortest_path(n_bytes) {
                    best_path.insert(self.end, ms);
                    let mut visit: VecDeque<(MapPoint<N, N>, usize)> = VecDeque::new();
                    visit.push_back((self.end, ms));
                    while let Some((byte, steps)) = visit.pop_front() {
                        if steps == 0 {
                            continue;
                        }
                        for neighbor in [Compass::N, Compass::E, Compass::S, Compass::W]
                            .iter()
                            .filter_map(|d| byte.neighbor(*d))
                        {
                            if let Some(ns) = seen.get(&neighbor) {
                                if *ns == steps - 1
                                    && best_path.insert(neighbor, steps - 1).is_none()
                                {
                                    visit.push_back((neighbor, steps - 1));
                                }
                            }
                        }
                    }
                } else {
                    return self.bytes[n_bytes - 1];
                }
            } else {
                let new_falling_byte = self.bytes[n_bytes];
                n_bytes += 1;
                if let Some(steps) = best_path.remove(&new_falling_byte) {
                    if !best_path.values().any(|s| *s == steps) {
                        // broken step chain, try to find new best path, which includes
                        // all bytes from 0 to including new_falling_byte
                        best_path.clear();
                    }
                }
            }
        }
        self.end
    }
}

// give by challenge
const N: usize = 71;

pub fn day_18() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_18.txt");
    let challenge = Day18Data::<N>::from(input);

    let (result_part1, _) = challenge.shortest_path(1024).unwrap();
    println!("result day 18 part 1: {}", result_part1);
    assert_eq!(result_part1, 506);

    let result_part2 = challenge.first_block(1024);
    println!(
        "result day 18 part 2: {},{}",
        result_part2.x(),
        result_part2.y()
    );
    assert_eq!(result_part2, (62, 6).into());

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // give by challenge
    const E: usize = 7;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_18_example.txt");
        let challenge = Day18Data::<E>::from(input);

        let (result_part1, _) = challenge.shortest_path(12).unwrap();
        println!("result day 18 part 1: {}", result_part1);
        assert_eq!(result_part1, 22);

        let result_part2 = challenge.first_block(12);
        println!("result day 18 part 2: {}", result_part2);
        assert_eq!(result_part2, (6, 1).into());

        Ok(())
    }
}
