//!day_06.rs

#[cfg(feature = "long-run-time")]
use crate::utilities::SnapshotHashSet;
use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::HashSet;

struct IterMap<'a, const N: usize> {
    map: &'a MyMap2D<char, N, N>,
    current_tile: MapPoint<N, N>,
    direction: Compass,
}

impl<'a, const N: usize> IterMap<'a, N> {
    fn new(map: &'a MyMap2D<char, N, N>, current_tile: MapPoint<N, N>, direction: Compass) -> Self {
        IterMap {
            map,
            current_tile,
            direction,
        }
    }
}

impl<'a, const N: usize> Iterator for IterMap<'a, N> {
    type Item = (MapPoint<N, N>, Compass);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_tile) = self.current_tile.neighbor(self.direction) {
            if *self.map.get(next_tile) != '#' {
                self.current_tile = next_tile;
                return Some((self.current_tile, self.direction));
            }
            self.direction = self.direction.clockwise().clockwise();
        }
        None
    }
}

#[derive(Debug)]
struct Day06Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    start_tile: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day06Data<N> {
    fn from(value: &str) -> Self {
        let map = MyMap2D::from(value);
        let (start_tile, _) = map.iter().find(|(_, c)| **c == '^').unwrap();
        Self { map, start_tile }
    }
}

impl<const N: usize> Day06Data<N> {
    fn count_visited_map_tiles(&self) -> (usize, Vec<(MapPoint<N, N>, Compass)>) {
        let mut visited_tiles: HashSet<MapPoint<N, N>> = HashSet::with_capacity(N);
        let mut path: Vec<(MapPoint<N, N>, Compass)> = Vec::with_capacity(N);
        visited_tiles.insert(self.start_tile);
        path.push((self.start_tile, Compass::N));
        let iter_map = IterMap::new(&self.map, self.start_tile, Compass::N);
        for (next_tile, next_direction) in iter_map {
            visited_tiles.insert(next_tile);
            path.push((next_tile, next_direction));
        }
        (visited_tiles.len(), path)
    }

    #[cfg(feature = "long-run-time")]
    fn count_possible_loop_blocks(&mut self, path: Vec<(MapPoint<N, N>, Compass)>) -> usize {
        let mut visited_tiles: SnapshotHashSet<(MapPoint<N, N>, Compass)> =
            SnapshotHashSet::with_capacity(N);
        let mut blocked_tiles: HashSet<MapPoint<N, N>> = HashSet::with_capacity(N);
        // insert start_tile to prevent it from being blocked during for loop
        blocked_tiles.insert(self.start_tile);
        let mut current_tile = self.start_tile;
        let mut current_direction = Compass::N;
        let mut loop_blocks = 0;
        for (next_tile, next_direction) in path.iter().skip(1) {
            visited_tiles.insert((current_tile, current_direction));
            // block next tile in path (if not blocked yet) and check if it results in a loop
            if blocked_tiles.insert(*next_tile) {
                // block next_tile
                self.map.set(*next_tile, '#');
                // set baseline of visited tiles. Without baseline it would be filled with visits
                // from map states, which do not exist anymore for the following cycles with other
                // blocked tiles. With baseline we can reset to clean state of visited tiles.
                visited_tiles.set_baseline();
                let check_loop_iter = IterMap::new(&self.map, current_tile, current_direction);
                for (check_tile, check_direction) in check_loop_iter {
                    if !visited_tiles.insert((check_tile, check_direction)) {
                        // new loop
                        loop_blocks += 1;
                        break;
                    }
                }
                visited_tiles.reset_to_baseline();
                // unblock next_tile
                self.map.set(*next_tile, '.');
            }
            current_tile = *next_tile;
            current_direction = *next_direction;
        }
        loop_blocks
    }
}

// taken from ../../../../aoc_input/aoc-2024/day_06.txt
const N: usize = 130;

pub fn day_06() -> Result<()> {
    println!("Happy Nikolaus!");
    let input = include_str!("../../../../aoc_input/aoc-2024/day_06.txt");

    #[cfg(feature = "long-run-time")]
    let mut challenge = Day06Data::<N>::from(input);

    #[cfg(not(feature = "long-run-time"))]
    let challenge = Day06Data::<N>::from(input);

    #[cfg(feature = "long-run-time")]
    let (result_part1, path) = challenge.count_visited_map_tiles();

    #[cfg(not(feature = "long-run-time"))]
    let (result_part1, _) = challenge.count_visited_map_tiles();

    println!("result day 06 part 1: {}", result_part1);
    assert_eq!(result_part1, 4_826);

    #[cfg(feature = "long-run-time")]
    {
        let result_part2 = challenge.count_possible_loop_blocks(path);
        println!("result day 06 part 2: {}", result_part2);
        assert_eq!(result_part2, 1_721);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 06 part 2 skipped because of long run time");
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // taken from ../../../../aoc_input/aoc-2024/day_06_example.txt
    const E: usize = 10;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_06_example.txt");
        let mut challenge = Day06Data::<E>::from(input);

        let (result_part1, path) = challenge.count_visited_map_tiles();
        println!("result day 06 part 1: {}", result_part1);
        assert_eq!(result_part1, 41);

        #[cfg(feature = "long-run-time")]
        {
            let result_part2 = challenge.count_possible_loop_blocks(path);
            println!("result day 06 part 2: {}", result_part2);
            assert_eq!(result_part2, 6);
        }
        Ok(())
    }
}
