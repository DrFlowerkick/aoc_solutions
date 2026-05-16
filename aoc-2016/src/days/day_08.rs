//!day_08.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

#[derive(Clone, Copy)]
struct Screen<const X: usize, const Y: usize> {
    screen: MyMap2D<char, X, Y>,
}

impl<const X: usize, const Y: usize> Screen<X, Y> {
    fn new() -> Self {
        Screen {
            screen: MyMap2D::init('.'),
        }
    }
    fn rect(&mut self, wide: usize, tall: usize) {
        for y in 0..tall {
            for x in 0..wide {
                self.screen.set((x, y).into(), '#');
            }
        }
    }
    fn rotate_row(&mut self, row: usize, shift: usize) {
        self.screen.get_row_mut(row).rotate_right(shift);
    }
    fn rotate_column(&mut self, col: usize, shift: usize) {
        let mut column = self.screen.get_column(col);
        column.rotate_right(shift);
        self.screen.apply_column(col, column);
    }
    fn count_on(&self) -> usize {
        self.screen.iter().filter(|(_, v)| **v == '#').count()
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some(size) = value.strip_prefix("rect ") {
            let (a, b) = size.split_once("x").unwrap();
            Instruction::Rect(a.parse().unwrap(), b.parse().unwrap())
        } else if let Some(row) = value.strip_prefix("rotate row y=") {
            let (a, b) = row.split_once(" by ").unwrap();
            Instruction::RotRow(a.parse().unwrap(), b.parse().unwrap())
        } else if let Some(col) = value.strip_prefix("rotate column x=") {
            let (a, b) = col.split_once(" by ").unwrap();
            Instruction::RotCol(a.parse().unwrap(), b.parse().unwrap())
        } else {
            panic!("unknown instruction")
        }
    }
}

impl Instruction {
    fn apply_to_screen<const X: usize, const Y: usize>(&self, screen: &mut Screen<X, Y>) {
        match *self {
            Instruction::Rect(wide, tall) => screen.rect(wide, tall),
            Instruction::RotRow(row, shift) => screen.rotate_row(row, shift),
            Instruction::RotCol(col, shift) => screen.rotate_column(col, shift),
        }
    }
}

struct ChallengeInput {
    instructions: Vec<Instruction>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            instructions: value.lines().map(Instruction::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1<const X: usize, const Y: usize>(&self, screen: &mut Screen<X, Y>) -> usize {
        for instruction in self.instructions.iter() {
            instruction.apply_to_screen(screen);
        }
        screen.count_on()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_08.txt");
    let challenge = ChallengeInput::from(input);

    let mut screen = Screen::<50, 6>::new();
    let result_part1 = challenge.solution_part_1(&mut screen);
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 110);

    println!("result day_08 part 2:\n{}", screen.screen);
    assert_eq!(
        format!("{}", screen.screen),
        "####...##.#..#.###..#..#..##..###..#....#...#..##.\n\
         ...#....#.#..#.#..#.#.#..#..#.#..#.#....#...#...#.\n\
         ..#.....#.####.#..#.##...#....#..#.#.....#.#....#.\n\
         .#......#.#..#.###..#.#..#....###..#......#.....#.\n\
         #....#..#.#..#.#.#..#.#..#..#.#....#......#..#..#.\n\
         ####..##..#..#.#..#.#..#..##..#....####...#...##.."
    );

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_08_example.txt");
        let example = ChallengeInput::from(input);

        let mut screen = Screen::<7, 3>::new();
        let result_part1 = example.solution_part_1(&mut screen);
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 6);

        println!("result day_08 part 2:\n{}", screen.screen);
        assert_eq!(
            format!("{}", screen.screen),
            ".#..#.#\n\
             #.#....\n\
             .#....."
        );

        Ok(())
    }
}
