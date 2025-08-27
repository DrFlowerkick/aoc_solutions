//!day_04.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

#[derive(Debug)]
struct Day04Data<const N: usize> {
    puzzle: MyMap2D<char, N, N>,
}

impl<const N: usize> From<&str> for Day04Data<N> {
    fn from(value: &str) -> Self {
        Self {
            puzzle: MyMap2D::from(value),
        }
    }
}

impl<const N: usize> Day04Data<N> {
    fn count_xmas(&self) -> usize {
        let mut count = 0;
        for (p_x, _) in self.puzzle.iter().filter(|(_, c)| **c == 'X') {
            for (_, o_m, _) in self
                .puzzle
                .iter_neighbors_with_corners(p_x)
                .filter(|(_, _, c)| **c == 'M')
            {
                let xmas: String = self
                    .puzzle
                    .iter_orientation(p_x, o_m)
                    .map(|(_, c)| c)
                    .take(4)
                    .collect();
                if xmas == "XMAS" {
                    count += 1;
                }
            }
        }
        count
    }
    fn count_cross_mas(&self) -> usize {
        let mut count = 0;
        for (p_x, _) in self.puzzle.iter().filter(|(_, c)| **c == 'A') {
            let cross: String = self
                .puzzle
                .iter_neighbors_with_corners(p_x)
                .filter(|(_, o, _)| o.is_ordinal())
                .map(|(_, _, c)| c)
                .collect();
            if ["MMSS", "MSSM", "SMMS", "SSMM"].contains(&cross.as_str()) {
                count += 1;
            }
        }
        count
    }
}

// taken from ../../../../aoc_input/aoc-2024/day_04.txt
const N: usize = 140;

pub fn day_04() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_04.txt");
    let challenge = Day04Data::<N>::from(input);

    let result_part1 = challenge.count_xmas();
    println!("result day 04 part 1: {}", result_part1);
    assert_eq!(result_part1, 2_427);

    let result_part2 = challenge.count_cross_mas();
    println!("result day 04 part 2: {}", result_part2);
    assert_eq!(result_part2, 1900);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // taken from ../../../../aoc_input/aoc-2024/day_04_example.txt
    const E: usize = 10;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_04_example.txt");
        let challenge = Day04Data::<E>::from(input);

        let result_part1 = challenge.count_xmas();
        println!("result day 04 part 1: {}", result_part1);
        assert_eq!(result_part1, 18);

        let result_part2 = challenge.count_cross_mas();
        println!("result day 04 part 2: {}", result_part2);
        assert_eq!(result_part2, 9);

        Ok(())
    }
}
