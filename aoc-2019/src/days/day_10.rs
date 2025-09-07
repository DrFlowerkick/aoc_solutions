//!day_10.rs

use anyhow::Result;
use my_lib::my_geometry::{
    my_line::LineSegment,
    my_point::{Cylindrical, Point},
};
use std::collections::{HashMap, HashSet};

struct ChallengeInput {
    asteroids: HashSet<Point>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut asteroids: HashSet<Point> = HashSet::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.insert((x as i64, y as i64).into());
                }
            }
        }
        ChallengeInput { asteroids }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (usize, i64) {
        let mut visible_asteroids: HashMap<Point, HashSet<Point>> = HashMap::new();
        for asteroid_1 in self.asteroids.iter() {
            for asteroid_2 in self.asteroids.iter().filter(|a| *a != asteroid_1) {
                let is_visible = if let Some(a) = visible_asteroids.get(asteroid_2)
                    && a.contains(asteroid_1)
                {
                    true
                } else {
                    let line = LineSegment::new(*asteroid_1, *asteroid_2);
                    !self
                        .asteroids
                        .iter()
                        .filter(|a| *a != asteroid_1 && *a != asteroid_2)
                        .any(|a| line == *a)
                };
                if is_visible {
                    // insert new visible asteroid
                    visible_asteroids
                        .entry(*asteroid_1)
                        .and_modify(|a| {
                            a.insert(*asteroid_2);
                        })
                        .or_insert({
                            let mut a = HashSet::new();
                            a.insert(*asteroid_2);
                            a
                        });
                }
            }
        }
        let (station, asteroids) = visible_asteroids
            .iter()
            .max_by_key(|(_, a)| a.len())
            .unwrap();
        // part 2
        if asteroids.len() < 200 {
            return (asteroids.len(), 0);
        }
        let mut asteroids_with_angle: Vec<(Point, f32)> = asteroids
            .iter()
            .map(|a| {
                // since positive y points down, angle rotates clockwise (as is required)
                // instead of counter clockwise. angle starts from positive x.
                // laser starts pointing up, which is at 270°.
                // therefore we have to rotate the angle back by 270° to start at laser.
                let c = Cylindrical::from(a.subtract(*station));
                let angle = (c.angle() - 270.0).rem_euclid(360.0);
                (*a, angle)
            })
            .collect();
        asteroids_with_angle.sort_by(|a, b| a.1.total_cmp(&b.1));
        // index 199 is 200th asteroid to be evaporated
        (
            asteroids.len(),
            asteroids_with_angle[199].0.x * 100 + asteroids_with_angle[199].0.y,
        )
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_10.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 253);

    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 815);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_10_example.txt");
        let solutions = [8, 33, 35, 41, 210];

        for (input, solution) in multi_input.split("\n\n").zip(solutions) {
            let example = ChallengeInput::from(input);

            let (result_part1, result_part2) = example.solution_part_1_and_2();
            println!("result day_10 part 1: {result_part1}");
            assert_eq!(result_part1, solution);

            if solution == 210 {
                println!("result day_10 part 2: {result_part2}");
                assert_eq!(result_part2, 802);
            }
        }

        Ok(())
    }
}
