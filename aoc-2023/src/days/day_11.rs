//!day_11.rs

use std::fmt::Display;

use anyhow::Result;
use my_lib::my_map_point::MapPoint;

// values taken from ../../../../aoc_input/aoc-2023/day_11.txt
// number of chars in one line
const X: usize = 140;
// number of lines
const Y: usize = 140;

#[derive(Default)]
struct Cosmos<const X: usize, const Y: usize> {
    galaxies: Vec<MapPoint<X, Y>>,
    empty_space_columns: Vec<usize>,
    empty_space_rows: Vec<usize>,
}

impl<const X: usize, const Y: usize> Display for Cosmos<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Y {
            for x in 0..X {
                let mp = MapPoint::<X, Y>::new(x, y);
                if self.galaxies.contains(&mp) {
                    write!(f, "#")?;
                } else if self.empty_space_columns.contains(&x)
                    || self.empty_space_rows.contains(&y)
                {
                    write!(f, "*")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const X: usize, const Y: usize> Cosmos<X, Y> {
    fn new(input: &str) -> Self {
        let mut cosmos = Cosmos::default();
        let mut empty_space_columns: Vec<bool> = vec![true; X];
        let mut empty_space_rows: Vec<bool> = vec![true; Y];
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    cosmos.galaxies.push(MapPoint::<X, Y>::new(x, y));
                    empty_space_columns[x] = false;
                    empty_space_rows[y] = false;
                }
            }
        }
        cosmos.empty_space_columns = empty_space_columns
            .iter()
            .enumerate()
            .filter(|(_, f)| **f)
            .map(|(x, _)| x)
            .collect();
        cosmos.empty_space_rows = empty_space_rows
            .iter()
            .enumerate()
            .filter(|(_, f)| **f)
            .map(|(y, _)| y)
            .collect();
        cosmos
    }
    fn calc_sum_galaxy_distances(&self, factor_empty_space: usize) -> usize {
        let mut distance = 0;
        for index_galaxy_1 in 0..(self.galaxies.len() - 1) {
            for index_galaxy_2 in (index_galaxy_1 + 1)..self.galaxies.len() {
                distance += self.calc_distance(
                    &self.galaxies[index_galaxy_1],
                    &self.galaxies[index_galaxy_2],
                    factor_empty_space,
                );
            }
        }
        distance
    }
    fn calc_distance(
        &self,
        alpha: &MapPoint<X, Y>,
        omega: &MapPoint<X, Y>,
        factor_empty_space: usize,
    ) -> usize {
        let min_x = alpha.x().min(omega.x());
        let max_x = alpha.x().max(omega.x());
        let min_y = alpha.y().min(omega.y());
        let max_y = alpha.y().max(omega.y());
        let distance = alpha.distance(*omega);
        let empty_spaces = self
            .empty_space_columns
            .iter()
            .filter(|x| min_x < **x && **x < max_x)
            .count()
            + self
                .empty_space_rows
                .iter()
                .filter(|y| min_y < **y && **y < max_y)
                .count();
        distance + (factor_empty_space.max(1) - 1) * empty_spaces
    }
}

pub fn day_11() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_11.txt");
    let cosmos = Cosmos::<X, Y>::new(input);
    let sum_distance = cosmos.calc_sum_galaxy_distances(2);
    println!("result day 11 part 1: {}", sum_distance);
    assert_eq!(sum_distance, 9_799_681);
    let sum_distance = cosmos.calc_sum_galaxy_distances(1_000_000);
    println!("result day 11 part 2: {}", sum_distance);
    assert_eq!(sum_distance, 513_171_773_355);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const XT: usize = 10;
    const YT: usize = 10;

    const TEST_INPUT: &str = "...#......
                              .......#..
                              #.........
                              ..........
                              ......#...
                              .#........
                              .........#
                              ..........
                              .......#..
                              #...#.....";

    #[test]
    fn test_distance() {
        let cosmos = Cosmos::<XT, YT>::new(TEST_INPUT);
        eprintln!("{}", cosmos);
        let sum_distance = cosmos.calc_sum_galaxy_distances(2);
        println!("result day 11 part 1: {}", sum_distance);
    }
}
