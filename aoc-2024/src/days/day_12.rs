//!day_12.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Day12Data<const N: usize> {
    garden: MyMap2D<char, N, N>,
}

impl<const N: usize> From<&str> for Day12Data<N> {
    fn from(value: &str) -> Self {
        Self {
            garden: MyMap2D::from(value),
        }
    }
}

impl<const N: usize> Day12Data<N> {
    fn get_fence_price(&self) -> usize {
        let mut price = 0;
        let mut seen: HashSet<MapPoint<N, N>> = HashSet::new();
        for (start_tile, _) in self.garden.iter() {
            if seen.insert(start_tile) {
                price += self.get_region_fence_price(start_tile, &mut seen);
            }
        }
        price
    }
    fn get_region_fence_price(
        &self,
        start_tile: MapPoint<N, N>,
        seen: &mut HashSet<MapPoint<N, N>>,
    ) -> usize {
        let mut tiles = 0;
        let mut fences = 0;
        let region = self.garden.get(start_tile);
        let mut visit: VecDeque<MapPoint<N, N>> = VecDeque::new();
        visit.push_back(start_tile);
        while let Some(current_tile) = visit.pop_front() {
            tiles += 1;
            let neighbors = [Compass::N, Compass::E, Compass::S, Compass::W];
            for neighbor in neighbors.iter().map(|n| current_tile.neighbor(*n)) {
                match neighbor {
                    Some(neighbor_tile) => {
                        if self.garden.get(neighbor_tile) == region {
                            if seen.insert(neighbor_tile) {
                                visit.push_back(neighbor_tile);
                            }
                        } else {
                            fences += 1;
                        }
                    }
                    None => fences += 1,
                }
            }
        }
        tiles * fences
    }

    fn get_fence_price_with_discount(&self) -> usize {
        let mut price = 0;
        let mut seen: HashSet<MapPoint<N, N>> = HashSet::new();
        for (start_tile, _) in self.garden.iter() {
            if seen.insert(start_tile) {
                price += self.get_region_fence_price_with_discount(start_tile, &mut seen);
            }
        }
        price
    }
    fn get_region_fence_price_with_discount(
        &self,
        start_tile: MapPoint<N, N>,
        seen: &mut HashSet<MapPoint<N, N>>,
    ) -> usize {
        let mut tiles = 0;
        let mut sides = 0;
        let region = self.garden.get(start_tile);
        let mut visit: VecDeque<MapPoint<N, N>> = VecDeque::new();
        visit.push_back(start_tile);
        while let Some(current_tile) = visit.pop_front() {
            tiles += 1;
            let neighbors = [Compass::N, Compass::E, Compass::S, Compass::W];
            let mut free_neighbors: HashSet<Compass> = HashSet::with_capacity(4);
            for (neighbor, orientation) in neighbors.iter().map(|n| (current_tile.neighbor(*n), n))
            {
                match neighbor {
                    Some(neighbor_tile) => {
                        if self.garden.get(neighbor_tile) == region {
                            if seen.insert(neighbor_tile) {
                                visit.push_back(neighbor_tile);
                            }
                        } else {
                            free_neighbors.insert(*orientation);
                        }
                    }
                    None => {
                        free_neighbors.insert(*orientation);
                    }
                }
            }
            // check corners
            let corners = [Compass::NE, Compass::SE, Compass::SW, Compass::NW];
            for (corner, orientation) in corners.iter().map(|c| (current_tile.neighbor(*c), c)) {
                let left = free_neighbors.contains(&orientation.counterclockwise());
                let right = free_neighbors.contains(&orientation.clockwise());

                if left && right {
                    // both sides of corner are free, therefore it is a "outside" corner or "connecting" corner
                    sides += 1;
                } else if !left
                    && !right
                    // both sides of corner are region tiles. If corner tile is free, it is a "inside" corner
                    && match corner {
                        Some(corner_tile) => self.garden.get(corner_tile) != region,
                        None => true,
                    }
                {
                    sides += 1;
                }
            }
        }
        tiles * sides
    }
}

// value taken from ../../../../aoc_input/aoc-2024/day_12.txt
const N: usize = 140;

pub fn day_12() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_12.txt");
    let challenge = Day12Data::<N>::from(input);

    let result_part1 = challenge.get_fence_price();
    println!("result day 12 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_485_656);

    let result_part2 = challenge.get_fence_price_with_discount();
    println!("result day 12 part 2: {}", result_part2);
    assert_eq!(result_part2, 899_196);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // value taken from ../../../../aoc_input/aoc-2024/day_12_example.txt
    const E: usize = 10;
    const S: usize = 6;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_12_example_01.txt");
        let challenge = Day12Data::<E>::from(input);

        let result_part1 = challenge.get_fence_price();
        println!("result day 12 part 1: {}", result_part1);
        assert_eq!(result_part1, 1_930);

        let input = include_str!("../../../../aoc_input/aoc-2024/day_12_example_02.txt");
        let challenge_02 = Day12Data::<S>::from(input);

        let result_part2_2 = challenge_02.get_fence_price_with_discount();
        println!("result day 12 part 2, example 2: {}", result_part2_2);
        assert_eq!(result_part2_2, 368);

        let result_part2 = challenge.get_fence_price_with_discount();
        println!("result day 12 part 2: {}", result_part2);
        assert_eq!(result_part2, 1_206);

        Ok(())
    }
}
