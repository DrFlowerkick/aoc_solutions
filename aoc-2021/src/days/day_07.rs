//!day_07.rs

use anyhow::Result;

struct Crabs {
    positions: Vec<i64>,
}

impl From<&str> for Crabs {
    fn from(value: &str) -> Self {
        Crabs {
            positions: value
                .split(',')
                .filter_map(|p| p.parse::<i64>().ok())
                .collect(),
        }
    }
}

impl Crabs {
    fn best_pos_fuel(&self, coarse_grid_percent: i64, part_two: bool) -> i64 {
        let mut min_pos = *self.positions.iter().min().unwrap();
        let max_pos = *self.positions.iter().max().unwrap();
        let step_size = (max_pos - min_pos) * coarse_grid_percent / 100;
        let mut best_fuel = i64::MAX;
        // do coarse grid search because at least in part one the function is not unimodal
        while min_pos < max_pos {
            let local_best_fuel = self.local_best_pos_ful(min_pos, min_pos + step_size, part_two);
            best_fuel = local_best_fuel.min(best_fuel);
            min_pos += step_size;
        }
        best_fuel
    }
    fn local_best_pos_ful(&self, mut min_pos: i64, mut max_pos: i64, part_two: bool) -> i64 {
        let mut min_pos_fuel = self.calc_fuel(min_pos, part_two);
        let mut max_pos_fuel = self.calc_fuel(max_pos, part_two);
        while min_pos != max_pos {
            if max_pos - min_pos == 1 {
                if max_pos_fuel < min_pos_fuel {
                    min_pos_fuel = max_pos_fuel;
                    min_pos = max_pos;
                } else {
                    max_pos_fuel = min_pos_fuel;
                    max_pos = min_pos;
                }
            } else {
                let middle_pos = (max_pos + min_pos) / 2;
                let middle_pos_fuel = self.calc_fuel(middle_pos, part_two);
                if (max_pos_fuel - middle_pos_fuel) < (min_pos_fuel - middle_pos_fuel) {
                    min_pos = middle_pos;
                    min_pos_fuel = middle_pos_fuel;
                } else {
                    max_pos = middle_pos;
                    max_pos_fuel = middle_pos_fuel;
                }
            }
        }
        min_pos_fuel
    }
    fn calc_fuel(&self, pos: i64, part_two: bool) -> i64 {
        if part_two {
            self.positions
                .iter()
                .map(|p| {
                    let delta = (*p - pos).abs();
                    (0..=delta).sum::<i64>()
                })
                .sum()
        } else {
            self.positions.iter().map(|p| (*p - pos).abs()).sum()
        }
    }
}

pub fn day_07() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_07.txt");
    let crabs = Crabs::from(input);

    let result_part1 = crabs.best_pos_fuel(20, false);
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, 354_129);

    let result_part2 = crabs.best_pos_fuel(100, true);
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 98_905_973);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_07_example.txt");
        let crabs = Crabs::from(input);

        let result_part1 = crabs.best_pos_fuel(20, false);
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 37);

        let result_part2 = crabs.best_pos_fuel(100, true);
        println!("result day_07 part 2: {result_part2}");
        assert_eq!(result_part2, 168);

        Ok(())
    }
}
