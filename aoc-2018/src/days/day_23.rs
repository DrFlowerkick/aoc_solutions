//!day_23.rs

use anyhow::Result;
use my_lib::{my_geometry::my_point::Point3D, my_pixels::my_box::Box3D};
use regex::Regex;
use std::collections::{BTreeSet, HashMap};

struct ChallengeInput {
    bots: HashMap<Point3D, i64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();

        ChallengeInput {
            bots: value
                .lines()
                .map(|l| {
                    let caps = re.captures(l).unwrap();
                    (
                        Point3D::new(
                            caps[1].parse().unwrap(),
                            caps[2].parse().unwrap(),
                            caps[3].parse().unwrap(),
                        ),
                        caps[4].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let (largest_bot, radius) = self.bots.iter().max_by_key(|(_, r)| **r).unwrap();
        self.bots
            .keys()
            .map(|p| p.delta(*largest_bot))
            .filter(|r| r <= radius)
            .count()
    }
    // we search position in boxes, starting in one big box including all bots,
    // than splitting the box in 8 boxes. Each box contains some bots. We look
    // for box in range of most bots with smallest size (size == 1) nearest
    // to center.
    // Using BTreeSet we look always for smallest element. Since we want most
    // bots in range, we invert this by looking for "least bots not in range".
    fn solution_part_2(&self) -> i64 {
        let max_dim = self
            .bots
            .keys()
            .map(|p| p.x.abs().max(p.y.abs()).max(p.z.abs()))
            .max()
            .unwrap();
        let initial_box = Box3D::new(
            Point3D::new(-max_dim, -max_dim, -max_dim),
            Point3D::new(max_dim, max_dim, max_dim),
        );
        let center = Point3D::default();
        let mut sorted_queue: BTreeSet<(usize, i64, i64, Box3D)> = BTreeSet::new();
        sorted_queue.insert((
            0,
            initial_box.size().unwrap(),
            initial_box.delta_to_point(center).unwrap(),
            initial_box,
        ));
        while let Some((_not_in_range, size, distance, search_box)) = sorted_queue.pop_first() {
            if size == 1 {
                // smallest possible box nearest to center
                return distance;
            }
            for next_box in search_box.split_box() {
                let in_range = self
                    .bots
                    .iter()
                    .filter(|(p, r)| next_box.delta_to_point(**p).unwrap() <= **r)
                    .count();
                if in_range > 0 {
                    // at least one bot in range
                    sorted_queue.insert((
                        self.bots.len() - in_range,
                        next_box.size().unwrap(),
                        next_box.delta_to_point(center).unwrap(),
                        next_box,
                    ));
                }
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_23.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, 713);

    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 104_501_042);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_23_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1: {result_part1}");
        assert_eq!(result_part1, 7);

        let input = include_str!("../../../../aoc_input/aoc-2018/day_23_example_part_2.txt");
        let example = ChallengeInput::from(input);
        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        assert_eq!(result_part2, 36);

        Ok(())
    }
}
