//!day_17.rs

use anyhow::Result;
use my_lib::my_geometry::{my_point::Point, my_rectangle::Rectangle};

struct ChallengeInput {
    target_area: Rectangle,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(", ").unwrap();
        let x = x.strip_prefix("target area: x=").unwrap();
        let x: Vec<i64> = x
            .split("..")
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        let y = y.strip_prefix("y=").unwrap();
        let y: Vec<i64> = y
            .split("..")
            .filter_map(|y| y.parse::<i64>().ok())
            .collect();
        let top_left = (x[0], y[1]).into();
        let bottom_right = (x[1], y[0]).into();
        ChallengeInput {
            target_area: Rectangle::new(top_left, bottom_right),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (i64, usize) {
        // brute force it at first try
        let mut min_v_x = 0;
        let mut x = 0;
        for v_x in 1..self.target_area.corners()[3].x {
            x += v_x;
            // [0] -> top left of rectangle
            if x >= self.target_area.corners()[0].x {
                break;
            }
            min_v_x = v_x;
        }
        // max_v_x is x of right side of target area
        let max_v_x = self.target_area.corners()[3].x;
        // min_v_y is bottom side of target area
        let min_v_y = self.target_area.corners()[3].y;
        // factor 2 of abs() of bottom of area seems to be good guess
        let max_v_y = self.target_area.corners()[0].y.abs() * 2;

        let mut target_hits = 0;
        let mut max_height = i64::MIN;
        for v_y in min_v_y..=max_v_y {
            for v_x in min_v_x..=max_v_x {
                if let Some(mh) = self.evaluate(v_x, v_y) {
                    target_hits += 1;
                    max_height = max_height.max(mh);
                }
            }
        }
        (max_height, target_hits)
    }

    fn evaluate(&self, v_x: i64, v_y: i64) -> Option<i64> {
        let mut current_point: Point = (0, 0).into();
        let mut velocity_vector: Point = (v_x, v_y).into();
        let mut max_height = i64::MIN;
        // [3] is bottom_right; target area < point: point is outside of rectangle
        while current_point.x < self.target_area.corners()[3].x
            && current_point.y > self.target_area.corners()[3].y
            && self.target_area < current_point
        {
            current_point = current_point.add(velocity_vector);
            max_height = max_height.max(current_point.y);
            velocity_vector.x = 0.max(velocity_vector.x - 1);
            velocity_vector.y -= 1;
        }
        if self.target_area < current_point {
            // did not hit target area
            return None;
        }
        Some(max_height)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_17.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, 13_203);

    println!("result day_17 part 2: {result_part2}");
    assert_eq!(result_part2, 5_644);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_17_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 45);

        println!("result day_17 part 2: {result_part2}");
        assert_eq!(result_part2, 112);

        Ok(())
    }
}
