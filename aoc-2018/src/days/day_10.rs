//!day_10.rs

use anyhow::Result;
use my_lib::my_geometry::{my_point::Point, my_rectangle::Rectangle};
use regex::Regex;

struct ChallengeInput {
    positions: Vec<Point>,
    velocities: Vec<Point>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut positions = Vec::new();
        let mut velocities = Vec::new();
        let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
            .unwrap();
        for line in value.lines() {
            let caps = re.captures(line).unwrap();
            let p_x = caps[1].parse().unwrap();
            let p_y = caps[2].parse().unwrap();
            let v_x = caps[3].parse().unwrap();
            let v_y = caps[4].parse().unwrap();
            positions.push(Point::new(p_x, p_y));
            velocities.push(Point::new(v_x, v_y));
        }
        ChallengeInput {
            positions,
            velocities,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&mut self, letter_height: i64) -> (String, u64) {
        let max_count = 1_000_000;
        let mut count = 0;
        while count < max_count {
            count += 1;
            let window = self.move_points();
            if window.size_y_inclusive() == letter_height {
                return (self.points_to_string(window), count);
            }
        }
        ("".into(), 0)
    }
    fn move_points(&mut self) -> Rectangle {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        for (point, velocity) in self.positions.iter_mut().zip(self.velocities.iter()) {
            *point = point.add(*velocity);
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
        let top_left = Point::new(min_x, max_y);
        let bottom_right = Point::new(max_x, min_y);
        Rectangle::new(top_left, bottom_right)
    }
    fn points_to_string(&self, window: Rectangle) -> String {
        let [top_left, _, _, bottom_right] = window.corners();
        let mut out = String::new();
        for y in bottom_right.y..=top_left.y {
            for x in top_left.x..=bottom_right.x {
                let point = Point::new(x, y);
                if self.positions.contains(&point) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            if y < top_left.y {
                out.push('\n');
            }
        }
        out
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_10.txt");
    let mut challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(10);
    println!("result day_10 part 1:\n{result_part1}");
    let solution_1 = "\
        #####...#........####...#....#..#....#..#####......###...####.\n\
        #....#..#.......#....#..##...#..#....#..#....#......#...#....#\n\
        #....#..#.......#.......##...#..#....#..#....#......#...#.....\n\
        #....#..#.......#.......#.#..#..#....#..#....#......#...#.....\n\
        #####...#.......#.......#.#..#..######..#####.......#...#.....\n\
        #....#..#.......#..###..#..#.#..#....#..#...........#...#.....\n\
        #....#..#.......#....#..#..#.#..#....#..#...........#...#.....\n\
        #....#..#.......#....#..#...##..#....#..#.......#...#...#.....\n\
        #....#..#.......#...##..#...##..#....#..#.......#...#...#....#\n\
        #####...######...###.#..#....#..#....#..#........###.....####.\
    ";
    assert_eq!(result_part1, solution_1);

    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 10_476);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_10_example.txt");
        let mut example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2(8);
        println!("result day_10 part 1:\n{result_part1}");
        let solution_part_1 = "\
            #...#..###\n\
            #...#...#.\n\
            #...#...#.\n\
            #####...#.\n\
            #...#...#.\n\
            #...#...#.\n\
            #...#...#.\n\
            #...#..###\
        ";
        assert_eq!(result_part1, solution_part_1);

        println!("result day_10 part 2: {result_part2}");
        assert_eq!(result_part2, 3);

        Ok(())
    }
}
