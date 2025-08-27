//!day_10.rs

use anyhow::Result;
use my_lib::{my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Day10Data<const N: usize> {
    map: MyMap2D<u8, N, N>,
}

impl<const N: usize> From<&str> for Day10Data<N> {
    fn from(value: &str) -> Self {
        let mut map: MyMap2D<u8, N, N> = MyMap2D::new();
        for (y, line) in value.lines().enumerate() {
            for (x, height) in line.chars().enumerate() {
                map.set((x, y).into(), height.to_digit(10).unwrap() as u8);
            }
        }
        Self { map }
    }
}

impl<const N: usize> Day10Data<N> {
    fn walk_trails_to_peaks(&self) -> (usize, usize) {
        self.map
            .iter()
            .filter(|(_, h)| **h == 0)
            .map(|(p, h)| self.walk_trail_from((p, *h, 1)))
            .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b))
    }
    fn walk_trail_from(&self, trail_head: (MapPoint<N, N>, u8, usize)) -> (usize, usize) {
        let mut trails: VecDeque<(MapPoint<N, N>, u8, usize)> = VecDeque::with_capacity(N);
        trails.push_back(trail_head);
        let mut seen: HashSet<(MapPoint<N, N>, u8)> = HashSet::with_capacity(N * N);
        let mut peaks = 0;
        let mut sum_rating = 0;
        while let Some((current_point, height, rating)) = trails.pop_front() {
            if height < 9 {
                for (next_point, _, next_height) in self
                    .map
                    .iter_neighbors(current_point)
                    .filter(|(_, _, h)| **h == height + 1)
                {
                    if seen.insert((next_point, *next_height)) {
                        trails.push_back((next_point, *next_height, rating));
                    } else {
                        let merge_rating =
                            trails.iter_mut().find(|(p, ..)| *p == next_point).unwrap();
                        merge_rating.2 += rating;
                    }
                }
            } else {
                peaks += 1;
                sum_rating += rating;
            }
        }
        (peaks, sum_rating)
    }
}

const N: usize = 43;

pub fn day_10() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_10.txt");
    let challenge = Day10Data::<N>::from(input);

    let (result_part1, result_part2) = challenge.walk_trails_to_peaks();
    println!("result day 10 part 1: {}", result_part1);
    assert_eq!(result_part1, 501);

    println!("result day 10 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_017);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const E: usize = 8;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_10_example.txt");
        let challenge = Day10Data::<E>::from(input);

        let (result_part1, result_part2) = challenge.walk_trails_to_peaks();
        println!("result day 10 part 1: {}", result_part1);
        assert_eq!(result_part1, 36);

        println!("result day 10 part 2: {}", result_part2);
        assert_eq!(result_part2, 81);

        Ok(())
    }
}
