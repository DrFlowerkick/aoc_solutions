//!day_09.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Ord, Eq)]
struct Height(u64);

impl From<char> for Height {
    fn from(value: char) -> Self {
        Height(value.to_digit(10).unwrap() as u64)
    }
}

struct ChallengeInput<const X: usize, const Y: usize> {
    height_map: MyMap2D<Height, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        ChallengeInput {
            height_map: MyMap2D::from(value),
        }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&self) -> u64 {
        self.height_map
            .iter()
            .filter(|(p, v)| self.height_map.iter_neighbors(*p).all(|(_, _, n)| *v < n))
            .map(|(_, v)| v.0 + 1)
            .sum()
    }
    fn solution_part_2(&self) -> usize {
        let low_points: Vec<_> = self
            .height_map
            .iter()
            .filter(|(p, v)| self.height_map.iter_neighbors(*p).all(|(_, _, n)| *v < n))
            .map(|(p, _)| p)
            .collect();
        let filter_fn = Box::new(
            |_point_of_next_cell: MapPoint<X, Y>,
             value_of_next_cell: &Height,
             _orientation_of_next_cell: Compass,
             _current_point: MapPoint<X, Y>,
             _value_of_current_cell: &Height,
             _current_distance: usize| { value_of_next_cell.0 < 9 },
        );
        let mut basin_sizes: Vec<usize> = Vec::with_capacity(low_points.len());
        for low_point in low_points {
            let size = self
                .height_map
                .iter_distance(low_point, filter_fn.clone())
                .count();
            basin_sizes.push(size);
        }
        basin_sizes.sort();
        basin_sizes[basin_sizes.len() - 3..].iter().product()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_09.txt");
    let challenge = ChallengeInput::<100, 100>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 558);

    let result_part2 = challenge.solution_part_2();
    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 882_942);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_09_example.txt");
        let example = ChallengeInput::<10, 5>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_09 part 1: {result_part1}");
        assert_eq!(result_part1, 15);

        let result_part2 = example.solution_part_2();
        println!("result day_09 part 2: {result_part2}");
        assert_eq!(result_part2, 1_134);

        Ok(())
    }
}
