//!day_25.rs

use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    t: i64,
}

impl From<&str> for Point4D {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',').map(|d| d.parse().unwrap());
        Point4D {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
            t: coords.next().unwrap(),
        }
    }
}

impl Point4D {
    fn delta(&self, other: &Point4D) -> i64 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }
}

struct ChallengeInput {
    points: Vec<Point4D>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            points: value.lines().map(Point4D::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        // first collect initial constellations
        let mut constellations: Vec<HashSet<Point4D>> = Vec::new();
        for (index, point_1) in self.points.iter().enumerate() {
            let mut constellation = HashSet::new();
            constellation.insert(*point_1);
            for point_2 in self.points.iter().skip(index + 1) {
                if point_1.delta(point_2) <= 3 {
                    constellation.insert(*point_2);
                }
            }
            constellations.push(constellation);
        }
        // merge constellations, which share points
        let mut merged_constellations: Vec<HashSet<Point4D>> = Vec::new();
        while let Some(mut constellation) = constellations.pop() {
            while let Some(merge_me_index) = constellations
                .iter()
                .position(|mm| constellation.intersection(mm).count() > 0)
            {
                for point in constellations.remove(merge_me_index) {
                    constellation.insert(point);
                }
            }
            merged_constellations.push(constellation);
        }
        merged_constellations.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_25.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_25 part 1: {result_part1}");
    assert_eq!(result_part1, 407);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_25() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_25_example.txt");

        let solutions = [2, 4, 3, 8];

        for (line, solution) in input.split("\n\n").zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_25 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }
}
