//!day_20.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Day20Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    start: MapPoint<N, N>,
    end: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day20Data<N> {
    fn from(value: &str) -> Self {
        let map: MyMap2D<char, N, N> = MyMap2D::from(value);
        let start = map
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(p, _)| p)
            .unwrap();
        let end = map
            .iter()
            .find(|(_, c)| **c == 'E')
            .map(|(p, _)| p)
            .unwrap();
        Self { map, start, end }
    }
}

impl<const N: usize> Day20Data<N> {
    fn calc_distance(&self) -> Vec<(MapPoint<N, N>, usize)> {
        let mut seen: HashSet<MapPoint<N, N>> = HashSet::with_capacity(N);
        seen.insert(self.start);
        let mut path: Vec<(MapPoint<N, N>, usize)> = Vec::with_capacity(N);
        path.push((self.start, 0));
        let mut visit: VecDeque<(MapPoint<N, N>, usize)> = VecDeque::new();
        visit.push_back((self.start, 0));
        while let Some((point, distance)) = visit.pop_front() {
            if point == self.end {
                break;
            }
            for neighbor in [Compass::N, Compass::E, Compass::W, Compass::S]
                .iter()
                .filter_map(|dir| point.neighbor(*dir))
                .filter(|p| *self.map.get(*p) != '#')
            {
                if seen.insert(neighbor) {
                    path.push((neighbor, distance + 1));
                    visit.push_back((neighbor, distance + 1));
                }
            }
        }
        path
    }
    fn find_cheats(&self, min_distance_reduction: usize, max_cheat_size: usize) -> usize {
        // path contains distance of each path element to start.
        // therefore the distance value is monotonically increasing in path
        let path = self.calc_distance();
        let mut count = 0;
        // Because we slice in 2nd for loop from index + 1, the following statements are always true
        // 1.) All distance values 'd' in second for loop are greater than 'distance'.
        // 2.) 'coordinate_distance' can have at most the same value as 'd - distance'.
        // 3.) Therefore it is save to calc 'd - distance - coordinate_distance'
        for (index, (path_tile, distance)) in path.iter().enumerate() {
            for _cheat in path[index + 1..].iter().filter(|(p, d)| {
                let coordinate_distance = path_tile.distance(*p);
                coordinate_distance <= max_cheat_size
                    && d - distance - coordinate_distance >= min_distance_reduction
            }) {
                count += 1;
            }
        }
        count
    }
}

const N: usize = 141;

pub fn day_20() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_20.txt");
    let challenge = Day20Data::<N>::from(input);

    let result_part1 = challenge.find_cheats(100, 2);
    println!("result day 20 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_393);

    let result_part2 = challenge.find_cheats(100, 20);
    println!("result day 20 part 2: {}", result_part2);
    assert_eq!(result_part2, 990_096);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const E: usize = 15;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_20_example.txt");
        let challenge = Day20Data::<E>::from(input);

        let result_part1 = challenge.find_cheats(6, 2);
        println!("result day 20 part 1: {}", result_part1);
        assert_eq!(result_part1, 16);

        let result_part2 = challenge.find_cheats(72, 20);
        println!("result day 20 part 2: {}", result_part2);
        assert_eq!(result_part2, 29);

        Ok(())
    }
}
