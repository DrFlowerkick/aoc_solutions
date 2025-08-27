//!day_25.rs

use anyhow::Result;
use my_lib::{my_map_point::MapPoint, my_map_two_dim::MyMap2D};

struct ChallengeInput<const X: usize, const Y: usize> {
    cucumbers: MyMap2D<char, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        ChallengeInput {
            cucumbers: MyMap2D::from(value),
        }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&mut self) -> u64 {
        let mut steps = 0;
        loop {
            steps += 1;
            let move_east = self.move_east();
            let move_west = self.move_south();
            if move_east && move_west {
                // no movement in either direction
                break;
            }
        }
        steps
    }
    fn move_east(&mut self) -> bool {
        let moving_cucumbers: Vec<(MapPoint<X, Y>, MapPoint<X, Y>)> = self
            .cucumbers
            .iter()
            .filter(|(_, c)| **c == '>')
            .filter_map(|(p, _)| {
                let neighbor = if p.x() == X - 1 {
                    MapPoint::<X, Y>::new(0, p.y())
                } else {
                    MapPoint::new(p.x() + 1, p.y())
                };
                if *self.cucumbers.get(neighbor) == '.' {
                    Some((p, neighbor))
                } else {
                    None
                }
            })
            .collect();
        if moving_cucumbers.is_empty() {
            return true;
        }
        self.do_moves(&moving_cucumbers);
        false
    }
    fn move_south(&mut self) -> bool {
        let moving_cucumbers: Vec<(MapPoint<X, Y>, MapPoint<X, Y>)> = self
            .cucumbers
            .iter()
            .filter(|(_, c)| **c == 'v')
            .filter_map(|(p, _)| {
                let neighbor = if p.y() == Y - 1 {
                    MapPoint::<X, Y>::new(p.x(), 0)
                } else {
                    MapPoint::new(p.x(), p.y() + 1)
                };
                if *self.cucumbers.get(neighbor) == '.' {
                    Some((p, neighbor))
                } else {
                    None
                }
            })
            .collect();
        if moving_cucumbers.is_empty() {
            return true;
        }
        self.do_moves(&moving_cucumbers);
        false
    }
    fn do_moves(&mut self, moving_cucumbers: &[(MapPoint<X, Y>, MapPoint<X, Y>)]) {
        for (old, new) in moving_cucumbers {
            self.cucumbers.swap_cell_values(*old, *new);
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_25.txt");
    let mut challenge = ChallengeInput::<139, 137>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_25 part 1: {result_part1}");
    assert_eq!(result_part1, 305);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_25() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_25_example.txt");
        let mut example = ChallengeInput::<10, 9>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_25 part 1: {result_part1}");
        assert_eq!(result_part1, 58);

        Ok(())
    }
}
