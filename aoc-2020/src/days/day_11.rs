//!day_11.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_two_dim::MyMap2D};

struct ChallengeInput<const X: usize, const Y: usize> {
    seats: MyMap2D<char, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        ChallengeInput {
            seats: MyMap2D::from(value),
        }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&self) -> usize {
        let mut last_state = MyMap2D::<char, X, Y>::default();
        let mut current_state = self.seats;
        while current_state != last_state {
            last_state = current_state;
            for (pos, seat) in last_state.iter().filter(|(_, s)| **s != '.') {
                let occupied_neighbors = last_state
                    .iter_neighbors_with_corners(pos)
                    .filter(|(_, _, s)| **s == '#')
                    .count();
                match seat {
                    '#' => {
                        if occupied_neighbors >= 4 {
                            current_state.set(pos, 'L');
                        }
                    }
                    'L' => {
                        if occupied_neighbors == 0 {
                            current_state.set(pos, '#');
                        }
                    }
                    _ => panic!("unknown char"),
                }
            }
        }

        current_state.iter().filter(|(_, s)| **s == '#').count()
    }
    fn solution_part_2(&self) -> usize {
        let mut last_state = MyMap2D::<char, X, Y>::default();
        let mut current_state = self.seats;
        while current_state != last_state {
            last_state = current_state;
            for (pos, seat) in last_state.iter().filter(|(_, s)| **s != '.') {
                let occupied_neighbors = Compass::from_u8(255)
                    .iter()
                    .filter_map(|orientation| {
                        last_state
                            .iter_orientation(pos, *orientation)
                            .skip(1)
                            .find(|(_, s)| **s != '.')
                    })
                    .filter(|(_, s)| **s == '#')
                    .count();
                match seat {
                    '#' => {
                        if occupied_neighbors >= 5 {
                            current_state.set(pos, 'L');
                        }
                    }
                    'L' => {
                        if occupied_neighbors == 0 {
                            current_state.set(pos, '#');
                        }
                    }
                    _ => panic!("unknown char"),
                }
            }
        }

        current_state.iter().filter(|(_, s)| **s == '#').count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_11.txt");
    let challenge = ChallengeInput::<91, 90>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, 2_113);

    let result_part2 = challenge.solution_part_2();
    println!("result day_11 part 2: {result_part2}");
    assert_eq!(result_part2, 1_865);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_11_example.txt");
        let example = ChallengeInput::<10, 10>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_11 part 1: {result_part1}");
        assert_eq!(result_part1, 37);

        let result_part2 = example.solution_part_2();
        println!("result day_11 part 2: {result_part2}");
        assert_eq!(result_part2, 26);

        Ok(())
    }
}
