//!day_14.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn move_seconds(&mut self, seconds: i64, width: i64, height: i64) {
        self.position = self.position.add(self.velocity.scale(seconds));
        self.position.x %= width;
        if self.position.x < 0 {
            self.position.x += width;
        }
        self.position.y %= height;
        if self.position.y < 0 {
            self.position.y += height;
        }
    }
}

#[derive(Debug, Clone)]
struct Day14Data {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl From<&str> for Day14Data {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
        Self {
            robots: re
                .captures_iter(value)
                .map(|cap| Robot {
                    position: (
                        cap[1].parse::<i64>().unwrap(),
                        cap[2].parse::<i64>().unwrap(),
                    )
                        .into(),
                    velocity: (
                        cap[3].parse::<i64>().unwrap(),
                        cap[4].parse::<i64>().unwrap(),
                    )
                        .into(),
                })
                .collect(),
            width: 101,
            height: 103,
        }
    }
}

impl Day14Data {
    fn move_seconds(&mut self, seconds: i64) -> i64 {
        let mid_height = self.height / 2;
        let mid_width = self.width / 2;
        let mut quadrants = [0_i64; 4];
        for robot in self.robots.iter_mut() {
            robot.move_seconds(seconds, self.width, self.height);
            let left_right = match robot.position.x.cmp(&mid_width) {
                Ordering::Greater => true,
                Ordering::Less => false,
                Ordering::Equal => continue,
            };
            let bottom_top = match robot.position.y.cmp(&mid_height) {
                Ordering::Greater => true,
                Ordering::Less => false,
                Ordering::Equal => continue,
            };
            let index_quadrant: usize = match (left_right, bottom_top) {
                (false, false) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (true, true) => 3,
            };
            quadrants[index_quadrant] += 1;
        }
        quadrants.iter().product()
    }
    fn search_the_tree(&mut self) -> i64 {
        let mut seconds = 0;
        let space_left = 20;
        let space_right = 20;
        let space_bottom = 10;
        let space_top = 30;
        let percentage = 60.0;
        loop {
            seconds += 1;
            self.move_seconds(1);
            let in_window = self
                .robots
                .iter()
                .filter(|r| {
                    r.position.x >= space_left
                        && r.position.x <= self.width - space_right
                        && r.position.y >= space_top
                        && r.position.y <= self.height - space_bottom
                })
                .count() as f64;
            if in_window * 100.0 / (self.robots.len() as f64) > percentage {
                return seconds;
            }
        }
    }
}

pub fn day_14() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_14.txt");
    let mut challenge = Day14Data::from(input);
    let mut tree_search = challenge.clone();

    let result_part1 = challenge.move_seconds(100);
    println!("result day 14 part 1: {}", result_part1);
    assert_eq!(result_part1, 215_987_200);

    let result_part2 = tree_search.search_the_tree();
    println!("result day 14 part 2: {}", result_part2);
    assert_eq!(result_part2, 8050);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fmt::Display;

    impl Display for Day14Data {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..self.height {
                for x in 0..self.width {
                    let current_tile = Point::from((x, y));
                    let count = self
                        .robots
                        .iter()
                        .filter(|r| r.position == current_tile)
                        .count();
                    if count == 0 {
                        write!(f, ".")?;
                    } else {
                        write!(f, "{count}")?;
                    }
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_14_example.txt");
        let mut challenge = Day14Data::from(input);
        challenge.width = 11;
        challenge.height = 7;
        println!("{challenge}");

        let result_part1 = challenge.move_seconds(100);

        println!("{challenge}");
        println!("result day 14 part 1: {}", result_part1);
        assert_eq!(result_part1, 12);
        /*
        No second test result to test code
        */
        Ok(())
    }
}
