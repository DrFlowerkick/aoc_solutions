//!day_19.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::HashMap;

struct ChallengeInput {
    start: Point,
    map: HashMap<Point, char>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut start = Point::default();
        let mut map = HashMap::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| !c.is_whitespace()) {
                let p = Point::new(x as i64, y as i64);
                if y == 0 {
                    start = p;
                }
                map.insert(p, c);
            }
        }
        ChallengeInput { start, map }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (String, u64) {
        let mut dir = Compass::S;
        let mut letters = String::new();
        let mut current = self.start;
        let mut steps = 0;
        'outer: loop {
            steps += 1;
            let c = self.map[&current];
            if c.is_alphabetic() {
                letters.push(c);
            }
            for new_dir in [
                dir,
                dir.clockwise().clockwise(),
                dir.counterclockwise().counterclockwise(),
            ] {
                let next = current.add(new_dir);
                if self.map.contains_key(&next) {
                    dir = new_dir;
                    current = next;
                    continue 'outer;
                }
            }
            // no next point to move to -> end of map
            break;
        }

        (letters, steps)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_19.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, "VTWBPYAQFU");

    println!("result day_19 part 2: {result_part2}");
    assert_eq!(result_part2, 17_358);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_19() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_19_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_19 part 1: {result_part1}");
        assert_eq!(result_part1, "ABCDEF");

        println!("result day_19 part 2: {result_part2}");
        assert_eq!(result_part2, 38);

        Ok(())
    }
}
