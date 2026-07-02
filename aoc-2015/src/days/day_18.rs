//!day_18.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashSet;

struct ChallengeInput {
    map: HashSet<Point>,
    max_x: i64,
    max_y: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut map = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in value.lines().enumerate() {
            let y = y as i64;
            max_y = max_y.max(y);
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                max_x = max_x.max(x);
                if c == '#' {
                    map.insert(Point::new(x, y));
                }
            }
        }
        ChallengeInput { map, max_x, max_y }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, steps: u64) -> usize {
        let mut map = self.map.clone();
        for _ in 0..steps {
            let mut next_map = HashSet::with_capacity(map.len());
            for y in 0..=self.max_y {
                for x in 0..=self.max_x {
                    let p = Point::new(x, y);
                    let on_neighbors = Compass::cardinals_and_ordinals()
                        .into_iter()
                        .map(|c| p.add(c))
                        .filter(|n| map.contains(n))
                        .count();
                    if on_neighbors == 3 || map.contains(&p) && on_neighbors == 2 {
                        next_map.insert(p);
                    }
                }
            }
            map = next_map;
        }
        map.len()
    }
    fn solution_part_2(&self, steps: u64) -> usize {
        let mut map = self.map.clone();
        let corners: HashSet<Point> = [
            (0, 0),
            (self.max_x, 0),
            (0, self.max_y),
            (self.max_x, self.max_y),
        ]
        .into_iter()
        .map(Point::from)
        .collect();
        for p in corners.iter().copied() {
            map.insert(p);
        }
        for _ in 0..steps {
            let mut next_map = HashSet::with_capacity(map.len());
            for y in 0..=self.max_y {
                for x in 0..=self.max_x {
                    let p = Point::new(x, y);
                    let on_neighbors = Compass::cardinals_and_ordinals()
                        .into_iter()
                        .map(|c| p.add(c))
                        .filter(|n| map.contains(n))
                        .count();
                    if corners.contains(&p)
                        || on_neighbors == 3
                        || map.contains(&p) && on_neighbors == 2
                    {
                        next_map.insert(p);
                    }
                }
            }
            map = next_map;
        }
        map.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_18.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(100);
    println!("result day_18 part 1: {result_part1}");
    assert_eq!(result_part1, 821);

    let result_part2 = challenge.solution_part_2(100);
    println!("result day_18 part 2: {result_part2}");
    assert_eq!(result_part2, 886);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_18_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(4);
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 4);

        let result_part2 = example.solution_part_2(5);
        println!("result day_18 part 2: {result_part2}");
        assert_eq!(result_part2, 17);

        Ok(())
    }
}
