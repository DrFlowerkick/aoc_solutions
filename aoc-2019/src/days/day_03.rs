//!day_03.rs

use anyhow::Result;
use my_lib::my_geometry::{my_line::LineSegment, my_point::Point};
use std::collections::HashMap;

struct Wire {
    wire: Vec<LineSegment>,
}

impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        let mut current = Point::new(0, 0);
        let mut wire: Vec<LineSegment> = Vec::new();
        for delta in value
            .split(',')
            .map(|d| (&d[0..1], &d[1..]))
            .filter_map(|(d, f)| {
                f.parse::<i64>().ok().map(|offset| match d {
                    "R" => Point::new(offset, 0),
                    "U" => Point::new(0, offset),
                    "L" => Point::new(-offset, 0),
                    "D" => Point::new(0, -offset),
                    _ => panic!("unknown direction"),
                })
            })
        {
            let next = current.add(delta);
            wire.push(LineSegment::new(current, next));
            current = next;
        }
        Self { wire }
    }
}

impl Wire {
    fn get_intersections(&self, other: &Self) -> HashMap<Point, i64> {
        let mut intersections: HashMap<Point, i64> = HashMap::new();
        let mut dist_1 = 0;
        let mut last_1 = Point::new(0, 0);
        for segment_1 in self.wire.iter() {
            let mut dist_2 = 0;
            let mut last_2 = Point::new(0, 0);
            for segment_2 in other.wire.iter() {
                if let Some(intersection) = segment_1.segment_intersection(segment_2)
                    && intersection != Point::new(0, 0)
                    && !intersections.contains_key(&intersection)
                {
                    intersections.insert(
                        intersection,
                        dist_1 + dist_2 + last_1.delta(intersection) + last_2.delta(intersection),
                    );
                }
                let [p1, p2] = segment_2.end_points();
                dist_2 += p1.delta(p2);
                last_2 = if p1 != last_2 { p1 } else { p2 };
            }
            let [p1, p2] = segment_1.end_points();
            dist_1 += p1.delta(p2);
            last_1 = if p1 != last_1 { p1 } else { p2 };
        }
        intersections
    }
}

struct ChallengeInput {
    wire_1: Wire,
    wire_2: Wire,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (wire_1, wire_2) = value.split_once('\n').unwrap();
        ChallengeInput {
            wire_1: Wire::from(wire_1),
            wire_2: Wire::from(wire_2),
        }
    }
}

impl ChallengeInput {
    fn solutions_part_1_and_2(&self) -> (i64, i64) {
        let intersections = self.wire_1.get_intersections(&self.wire_2);

        (
            intersections.keys().map(|p| p.delta((0, 0))).min().unwrap(),
            *intersections.values().min().unwrap(),
        )
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solutions_part_1_and_2();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 293);

    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 27_306);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_03_example.txt");
        let solutions_part_1_and_2 = [(6, 30), (159, 610), (135, 410)];

        for (input, (solution_part1, solution_part2)) in
            multi_input.split("\n\n").zip(solutions_part_1_and_2)
        {
            let example = ChallengeInput::from(input);

            let (result_part1, result_part2) = example.solutions_part_1_and_2();
            println!("result day_03 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part1);

            println!("result day_03 part 2: {result_part2}");
            assert_eq!(result_part2, solution_part2);
        }

        Ok(())
    }
}
