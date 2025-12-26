//!day_03.rs

use anyhow::Result;
use my_lib::{my_geometry::my_point::Point3D, my_pixels::my_box::Box3D};
use std::collections::VecDeque;

struct ChallengeInput {
    plans: Vec<Box3D>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            plans: value
                .lines()
                .map(|l| {
                    let (_, rec) = l.split_once(" @ ").unwrap();
                    let (point, size) = rec.split_once(": ").unwrap();
                    let (x, y) = point.split_once(',').unwrap();
                    let point = Point3D::new(x.parse().unwrap(), y.parse().unwrap(), 0);
                    let (width, height) = size.split_once('x').unwrap();
                    let delta = Point3D::new(
                        width.parse::<i64>().unwrap() - 1,
                        height.parse::<i64>().unwrap() - 1,
                        0,
                    );
                    Box3D::new(point, point.add(delta))
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut intersections: VecDeque<Box3D> = VecDeque::new();
        for (index, box_a) in self.plans.iter().enumerate() {
            for box_b in self.plans.iter().skip(index + 1) {
                if let Some(intersection) = box_a.intersect(*box_b) {
                    intersections.push_back(intersection);
                }
            }
        }
        let mut unique_plans: Vec<Box3D> = Vec::with_capacity(intersections.len() * 2);
        while let Some(intersection) = intersections.pop_front() {
            if let Some(plan_intersection) =
                unique_plans.iter().find_map(|p| p.intersect(intersection))
            {
                for remaining in intersection.subtract(plan_intersection) {
                    intersections.push_back(remaining);
                }
                continue;
            }
            unique_plans.push(intersection);
        }
        unique_plans.iter().filter_map(|p| p.size()).sum()
    }
    fn solution_part_2(&self) -> usize {
        'loop_a: for (index, box_a) in self.plans.iter().enumerate() {
            for box_b in self.plans.iter() {
                if box_a == box_b {
                    continue;
                }
                if box_a.intersect(*box_b).is_some() {
                    continue 'loop_a;
                }
            }
            return index + 1;
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 110195);

    let result_part2 = challenge.solution_part_2();
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 894);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_03_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_03 part 1: {result_part1}");
        assert_eq!(result_part1, 4);

        let result_part2 = example.solution_part_2();
        println!("result day_03 part 2: {result_part2}");
        assert_eq!(result_part2, 3);

        Ok(())
    }
}
