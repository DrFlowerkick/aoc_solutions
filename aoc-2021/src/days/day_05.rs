//!day_05.rs

use anyhow::Result;
use my_lib::my_geometry::{my_line::LineSegment, my_point::Point};
use std::collections::HashMap;

struct LineGrid {
    lines: Vec<LineSegment>,
}

impl From<&str> for LineGrid {
    fn from(input: &str) -> Self {
        let lines: Vec<LineSegment> = input
            .lines()
            .filter_map(|l| l.split_once(" -> "))
            .map(|(a, b)| (Point::try_from(a).unwrap(), Point::try_from(b).unwrap()))
            .map(|(a, b)| LineSegment::new(a, b))
            .collect();
        Self { lines }
    }
}

impl LineGrid {
    fn overlapping_lines(&self, exclude_diagonales: bool) -> usize {
        let mut grid: HashMap<Point, usize> = HashMap::new();
        for line in self.lines.iter() {
            if exclude_diagonales {
                let end_points = line.end_points();
                if end_points[0].x != end_points[1].x && end_points[0].y != end_points[1].y {
                    continue;
                }
            }
            for point in line.lattice_points_on_segment().iter() {
                grid.entry(*point).and_modify(|n| *n += 1).or_insert(1);
            }
        }
        grid.iter().filter(|(_, n)| **n > 1).count()
    }
}

pub fn day_05() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_05.txt");
    let line_grid = LineGrid::from(input);

    let result_part1 = line_grid.overlapping_lines(true);
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 4_655);

    let result_part2 = line_grid.overlapping_lines(false);
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 20_500);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_05_example.txt");
        let line_grid = LineGrid::from(input);

        let result_part1 = line_grid.overlapping_lines(true);
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        let result_part2 = line_grid.overlapping_lines(false);
        println!("result day_05 part 2: {result_part2}");
        assert_eq!(result_part2, 12);

        Ok(())
    }
}
