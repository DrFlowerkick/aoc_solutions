//!day_07.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::{HashMap, HashSet};

struct ChallengeInput {
    start: Point,
    splitters: HashSet<Point>,
    max_y: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        // use "y / 2" to ignore empty lines
        ChallengeInput {
            start: value
                .lines()
                .next()
                .and_then(|l| {
                    l.chars()
                        .position(|c| c == 'S')
                        .map(|x| Point::new(x as i64, 0))
                })
                .unwrap(),
            splitters: value
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '^')
                        .map(move |(x, _)| Point::new(x as i64, (y / 2) as i64))
                })
                .collect(),
            max_y: (value.lines().count() / 2) as i64,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut active_splitters: HashSet<Point> = HashSet::new();
        let mut beams: HashSet<Point> = HashSet::new();
        beams.insert(self.start);
        'line_loop: loop {
            let mut next_beams: HashSet<Point> = HashSet::with_capacity(beams.len() * 2);
            for beam in beams.iter() {
                let beam = beam.add((0, 1));
                if beam.y == self.max_y {
                    break 'line_loop;
                }
                if self.splitters.contains(&beam) {
                    active_splitters.insert(beam);
                    next_beams.insert(beam.add((1, 0)));
                    next_beams.insert(beam.add((-1, 0)));
                } else {
                    next_beams.insert(beam);
                }
            }
            beams = next_beams;
        }

        active_splitters.len()
    }
    fn solution_part_2(&self) -> u64 {
        let mut seen: HashMap<Point, u64> = HashMap::new();
        self.time_line_traversal(self.start, &mut seen)
    }
    fn time_line_traversal(&self, beam: Point, seen: &mut HashMap<Point, u64>) -> u64 {
        let beam = beam.add((0, 1));
        if beam.y == self.max_y {
            return 1;
        }
        if let Some(time_lines) = seen.get(&beam) {
            return *time_lines;
        }
        let mut time_lines = 0;
        if self.splitters.contains(&beam) {
            time_lines += self.time_line_traversal(beam.add((1, 0)), seen)
                + self.time_line_traversal(beam.add((-1, 0)), seen);
        } else {
            time_lines += self.time_line_traversal(beam, seen);
        }
        seen.insert(beam, time_lines);
        time_lines
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_07.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, 1_600);

    let result_part2 = challenge.solution_part_2();
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 8_632_253_783_011);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_07_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 21);

        let result_part2 = example.solution_part_2();
        println!("result day_07 part 2: {result_part2}");
        assert_eq!(result_part2, 40);

        Ok(())
    }
}
