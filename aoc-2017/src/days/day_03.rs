//!day_03.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashMap;

struct ChallengeInput {
    square: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            square: value.parse().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let (bottom_right, radius) = self.get_bottom_right();
        let pos_of_square = self.spiral_until_square(bottom_right, radius);
        pos_of_square.delta((0, 0))
    }
    fn solution_part_2(&self) -> i64 {
        let mut pos = Point::new(0, 0);
        let mut next_bottom_right = pos;
        let mut current_radius = 0;
        let mut max_steps = 0;
        let mut direction = Compass::N;
        let mut square_values: HashMap<Point, i64> = HashMap::new();
        square_values.insert(pos, 1);
        loop {
            if pos == next_bottom_right {
                next_bottom_right = pos.add((1, -1));
                pos = next_bottom_right;
                current_radius += 1;
                max_steps = current_radius * 2;
            }
            for _ in 0..max_steps {
                // Compass converts to Point with N: (0, -1), because most maps in aoc count positive moving S.
                // Therefore we cannot add direction directly to pos.
                let offset = match direction {
                    Compass::N => (0, 1),
                    Compass::W => (-1, 0),
                    Compass::S => (0, -1),
                    Compass::E => (1, 0),
                    _ => unreachable!(),
                };
                pos = pos.add(offset);
                let current_square_value = Compass::cardinals_and_ordinals()
                    .into_iter()
                    .filter_map(|n| square_values.get(&pos.add(n)))
                    .sum();
                if current_square_value > self.square {
                    return current_square_value;
                }
                square_values.insert(pos, current_square_value);
            }
            direction = direction.counterclockwise().counterclockwise();
        }
    }
    fn get_bottom_right(&self) -> (i64, i64) {
        let mut radius: i64 = 0;
        loop {
            let size = (radius * 2 + 1).pow(2);
            if size < self.square {
                radius += 1;
            } else if size == self.square {
                return (size, radius);
            } else {
                // reduce radius by one and calc size
                radius -= 1;
                return ((radius * 2 + 1).pow(2), radius);
            }
        }
    }
    fn spiral_until_square(&self, bottom_right: i64, radius: i64) -> Point {
        // start at bottom right of inner square area
        let mut pos = Point::new(radius, -radius);
        let mut current_square = bottom_right;
        let mut direction = Compass::N;
        // increment radius of inner square area to get max steps we can move from on corner
        // to next corner of outer square
        let max_steps = (radius + 1) * 2;
        loop {
            if current_square == self.square {
                return pos;
            }
            if direction == Compass::N {
                // move pos to outer square bottom right
                pos = pos.add((1, -1));
            }
            let distance = max_steps.min(self.square - current_square);
            let delta = match direction {
                Compass::N => (0, distance),
                Compass::W => (-distance, 0),
                Compass::S => (0, -distance),
                Compass::E => (distance, 0),
                _ => unreachable!(),
            };
            pos = pos.add(delta);
            current_square += distance;
            direction = direction.counterclockwise().counterclockwise();
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 475);

    let result_part2 = challenge.solution_part_2();
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 279_138);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_03_example_part_1.txt");
        let solutions = [0, 3, 2, 31];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_03 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }

    #[test]
    fn test_example_day_03_part_2() -> Result<()> {
        let example = ChallengeInput::from("750");

        let result_part2 = example.solution_part_2();
        println!("result day_03 part 2: {result_part2}");
        assert_eq!(result_part2, 806);

        Ok(())
    }
}
