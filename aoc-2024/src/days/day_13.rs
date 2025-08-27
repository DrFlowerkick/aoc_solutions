//!day_13.rs

use anyhow::Result;
use my_lib::my_geometry::{my_line::Line, my_point::Point};
use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
        let mut regex_iter = re.captures_iter(value).map(|cap| {
            let x = cap[1].parse::<i64>().unwrap();
            let y = cap[2].parse::<i64>().unwrap();
            Point::from((x, y))
        });
        Self {
            button_a: regex_iter.next().unwrap(),
            button_b: regex_iter.next().unwrap(),
            prize: regex_iter.next().unwrap(),
        }
    }
}

impl ClawMachine {
    fn calc_tokens(&self, off_set_prize: i64) -> Option<i64> {
        // handle this as two linear functions or lines, which have to intersect at prize coordinates
        let line_x = Line::new(
            self.button_a.x,
            self.button_b.x,
            -self.prize.x - off_set_prize,
        );
        let line_y = Line::new(
            self.button_a.y,
            self.button_b.y,
            -self.prize.y - off_set_prize,
        );
        if let Some(intersection) = line_x.line_intersection(&line_y) {
            // check if intersection is on both lines
            // This is required, because line:intersection() calculates float result
            // if integer solution is not possible and converts it back to integer
            // This may result in intersections with slight offset from lines.
            if line_x == intersection && line_y == intersection {
                return Some(intersection.x * 3 + intersection.y);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Day13Data {
    arcade: Vec<ClawMachine>,
}

impl From<&str> for Day13Data {
    fn from(value: &str) -> Self {
        Self {
            arcade: value.split("\n\n").map(ClawMachine::from).collect(),
        }
    }
}

impl Day13Data {
    fn calc_tokens(&self, off_set_prize: i64) -> i64 {
        self.arcade
            .iter()
            .filter_map(|cm| cm.calc_tokens(off_set_prize))
            .sum()
    }
}

pub fn day_13() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_13.txt");
    let challenge = Day13Data::from(input);

    let result_part1 = challenge.calc_tokens(0);
    println!("result day 13 part 1: {}", result_part1);
    assert_eq!(result_part1, 25_629);

    let result_part2 = challenge.calc_tokens(10_000_000_000_000);
    println!("result day 13 part 2: {}", result_part2);
    assert_eq!(result_part2, 107_487_112_929_999);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_13_example.txt");
        let challenge = Day13Data::from(input);

        let result_part1 = challenge.calc_tokens(0);
        println!("result day 13 part 1: {}", result_part1);
        assert_eq!(result_part1, 480);

        let result_part2 = challenge.calc_tokens(10_000_000_000_000);
        println!("result day 13 part 2: {}", result_part2);
        assert_eq!(result_part2, 875_318_608_908);

        Ok(())
    }
}
