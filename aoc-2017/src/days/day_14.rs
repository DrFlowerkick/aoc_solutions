//!day_14.rs

use super::day_10::KnotHash;
use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::{HashSet, VecDeque};

struct ChallengeInput {
    key: String,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput { key: value.into() }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (usize, usize) {
        // part 1
        let mut squares: HashSet<Point> = HashSet::new();
        for y in 0..128 {
            let line = format!("{}-{}", self.key, y);
            let knot_hash = KnotHash::new().knot_hash(line);
            for x in 0..128 {
                if knot_hash & 1_u128.rotate_left(x) > 0 {
                    squares.insert(Point::new(x as i64, y));
                }
            }
        }
        // part 2
        let mut seen: HashSet<Point> = HashSet::new();
        let mut regions = 0;
        for square in squares.iter() {
            if seen.insert(*square) {
                let mut queue: VecDeque<Point> = VecDeque::new();
                queue.push_back(*square);
                while let Some(current) = queue.pop_front() {
                    for neighbor in [(0, 1), (1, 0), (0, -1), (-1, 0)]
                        .into_iter()
                        .map(|n| current.add(n))
                        .filter(|n| squares.contains(n))
                    {
                        if seen.insert(neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
                regions += 1;
            }
        }
        (squares.len(), regions)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_14.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, 8_222);

    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 1_086);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_14() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_14_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_14 part 1: {result_part1}");
        assert_eq!(result_part1, 8_108);

        println!("result day_14 part 2: {result_part2}");
        assert_eq!(result_part2, 1_242);

        Ok(())
    }
}
