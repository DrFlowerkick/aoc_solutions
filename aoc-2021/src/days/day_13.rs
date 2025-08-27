//!day_13.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::{HashSet, VecDeque};

enum FoldAlong {
    X(i64),
    Y(i64),
}

struct ChallengeInput {
    dots: HashSet<Point>,
    fold_instructions: VecDeque<FoldAlong>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (dots, fold_instructions) = value.split_once("\n\n").unwrap();
        let dots = dots
            .lines()
            .filter_map(|c| Point::try_from(c).ok())
            .collect();
        let fold_instructions = fold_instructions
            .lines()
            .map(|fi| {
                let (dir, value) = fi.split_once('=').unwrap();
                let value = value.parse::<i64>().unwrap();
                match dir {
                    "fold along x" => FoldAlong::X(value),
                    "fold along y" => FoldAlong::Y(value),
                    _ => panic!("unknown fold instruction"),
                }
            })
            .collect();
        ChallengeInput {
            dots,
            fold_instructions,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        self.fold_once();
        self.dots.len()
    }
    fn fold_once(&mut self) {
        self.dots = match self.fold_instructions.pop_front().unwrap() {
            FoldAlong::X(axis) => self
                .dots
                .iter()
                .map(|p| {
                    if p.x > axis {
                        let x = axis - (p.x - axis);
                        (x, p.y).into()
                    } else {
                        *p
                    }
                })
                .collect(),
            FoldAlong::Y(axis) => self
                .dots
                .iter()
                .map(|p| {
                    if p.y > axis {
                        let y = axis - (p.y - axis);
                        (p.x, y).into()
                    } else {
                        *p
                    }
                })
                .collect(),
        }
    }
    fn solution_part_2(&mut self) -> String {
        while !self.fold_instructions.is_empty() {
            self.fold_once();
        }
        let min_x = self.dots.iter().min_by_key(|p| p.x).unwrap().x;
        let max_x = self.dots.iter().max_by_key(|p| p.x).unwrap().x;
        let min_y = self.dots.iter().min_by_key(|p| p.y).unwrap().y;
        let max_y = self.dots.iter().max_by_key(|p| p.y).unwrap().y;
        let mut letters = String::from("\n");
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dot = if self.dots.contains(&(x, y).into()) {
                    '#'
                } else {
                    '.'
                };
                letters.push(dot);
            }
            if y < max_y {
                letters.push('\n');
            }
        }
        letters
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_13.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 664);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    let expected = "\n\
        ####.####...##.#..#.####.#....###..#...\n\
        #....#.......#.#.#.....#.#....#..#.#...\n\
        ###..###.....#.##.....#..#....###..#...\n\
        #....#.......#.#.#...#...#....#..#.#...\n\
        #....#....#..#.#.#..#....#....#..#.#...\n\
        ####.#.....##..#..#.####.####.###..####";
    assert_eq!(result_part2, expected);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_13_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, 17);

        let result_part2 = example.solution_part_2();
        println!("result day_13 part 2: {result_part2}");
        let expected = "\n\
            #####\n\
            #...#\n\
            #...#\n\
            #...#\n\
            #####";
        assert_eq!(result_part2, expected);

        Ok(())
    }
}
