//!day_03.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::HashSet;

struct ChallengeInput {
    trees: HashSet<Point>,
    size_right: i64,
    size_down: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut size_right = 0;
        let mut size_down = 0;
        let mut trees: HashSet<Point> = HashSet::new();
        for (y, line) in value.lines().enumerate() {
            size_down = size_down.max(y);
            for (x, c) in line.chars().enumerate() {
                size_right = size_right.max(x);
                if c == '#' {
                    trees.insert(Point::new(x as i64, y as i64));
                }
            }
        }
        ChallengeInput {
            trees,
            size_right: (size_right + 1) as i64,
            size_down: (size_down + 1) as i64,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let vector = Point::new(3, 1);
        self.count_trees(vector)
    }
    fn solution_part_2(&self) -> u64 {
        let vectors = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        vectors
            .into_iter()
            .map(|v| self.count_trees(v.into()))
            .product()
    }
    fn count_trees(&self, vector: Point) -> u64 {
        let mut position = Point::new(0, 0);
        let mut tree_count = 0;
        while position.y < self.size_down {
            if self.trees.contains(&position) {
                tree_count += 1;
            }
            position = position.add(vector);
            position.x %= self.size_right;
        }
        tree_count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 156);

    let result_part2 = challenge.solution_part_2();
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 3_521_829_480);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_03_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_03 part 1: {result_part1}");
        assert_eq!(result_part1, 7);

        let result_part2 = example.solution_part_2();
        println!("result day_03 part 2: {result_part2}");
        assert_eq!(result_part2, 336);

        Ok(())
    }
}
