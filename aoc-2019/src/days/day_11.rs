//!day_11.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::HashMap;
use std::fmt::Write;

struct ChallengeInput {
    code: IntCodeComputer,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            code: IntCodeComputer::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut painted: HashMap<Point, i64> = HashMap::new();
        if let Err(err) = self.execute_paint_job(&mut painted) {
            panic!("some int code computer error: {err}");
        }
        painted.len()
    }
    fn solution_part_2(&self) -> String {
        let mut painted: HashMap<Point, i64> = HashMap::new();
        // start with white tile
        painted.insert(Point::new(0, 0), 1);
        if let Err(err) = self.execute_paint_job(&mut painted) {
            panic!("some int code computer error: {err}");
        }
        let min_x = painted.keys().map(|p| p.x).min().unwrap();
        let max_x = painted.keys().map(|p| p.x).max().unwrap();
        let min_y = painted.keys().map(|p| p.y).min().unwrap();
        let max_y = painted.keys().map(|p| p.y).max().unwrap();
        let mut out = String::new();
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if let Some(color) = painted.get(&(x, y).into())
                    && *color == 1
                {
                    write!(&mut out, "#").unwrap();
                } else {
                    write!(&mut out, " ").unwrap();
                }
            }
            if y > min_y {
                writeln!(&mut out).unwrap();
            }
        }
        out
    }
    fn execute_paint_job(&self, painted: &mut HashMap<Point, i64>) -> Result<(), String> {
        let mut paint_bot = self.code.clone();
        let mut current = Point::new(0, 0);
        let mut current_direction = '^';
        while let Some(color) = paint_bot.run_int_code(&[*painted.get(&current).unwrap_or(&0)])? {
            let direction = paint_bot
                .run_int_code(&[])?
                .ok_or(String::from("did not output direction"))?;
            painted
                .entry(current)
                .and_modify(|v| *v = color)
                .or_insert(color);
            current_direction = match (current_direction, direction) {
                ('^', 0) => '<',
                ('^', 1) => '>',
                ('<', 0) => 'v',
                ('<', 1) => '^',
                ('v', 0) => '>',
                ('v', 1) => '<',
                ('>', 0) => '^',
                ('>', 1) => 'v',
                _ => unreachable!(),
            };
            let direction = match current_direction {
                '^' => (0, 1),
                '<' => (-1, 0),
                'v' => (0, -1),
                '>' => (1, 0),
                _ => unreachable!(),
            };
            current = current.add(direction.into());
        }
        Ok(())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, 2_339);

    let result_part2 = challenge.solution_part_2();
    println!("result day_11 part 2:\n{result_part2}");

    let solution = include_str!("../../../../aoc_input/aoc-2019/day_11_expected_part_2.txt");
    assert_eq!(result_part2, solution);

    Ok(())
}

#[cfg(test)]
mod tests {
    // no usable example provided in part 1
    /*use super::*;

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_11_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_11 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_11 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }*/
}
