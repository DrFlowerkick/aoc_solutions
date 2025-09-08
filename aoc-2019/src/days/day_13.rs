//!day_13.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::cmp::Ordering;
use std::collections::HashMap;

struct ChallengeInput {
    code: IntCodeComputer,
    score: i64,
    arcade: HashMap<Point, i64>,
    ball: Point,
    paddle: Point,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            code: IntCodeComputer::from(value),
            score: 0,
            arcade: HashMap::new(),
            ball: (0, 0).into(),
            paddle: (0, 0).into(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        if let Err(err) = self.generate_arcade() {
            panic!("{err}");
        }
        self.arcade.values().filter(|c| **c == 2).count()
    }
    fn solution_part_2(&mut self) -> i64 {
        if let Err(err) = self.run_game() {
            panic!("{err}");
        }
        self.score
    }
    fn generate_arcade(&mut self) -> Result<(), String> {
        let mut arcade_printer = self.code.clone();
        while let Some(x) = arcade_printer.run_int_code(&[])? {
            let y = arcade_printer.run_int_code(&[])?.expect("expected y");
            let id = arcade_printer.run_int_code(&[])?.expect("expected id");
            let point = (x, y).into();
            if id == 3 {
                self.paddle = point;
            } else if id == 4 {
                self.ball = point;
            } else {
                self.arcade.insert(point, id);
            }
        }
        Ok(())
    }
    fn run_game(&mut self) -> Result<(), String> {
        self.code.set_address(0, 2);
        let mut first_score = false;
        let mut last_score = false;
        let mut input = 0;
        while let Some(x) = self.code.run_int_code(&[input])? {
            let y = self.code.run_int_code(&[input])?.expect("expected y");
            let id = self.code.run_int_code(&[input])?.expect("expected id");

            if x == -1 && y == 0 {
                self.score = id;
                first_score = true;
                if last_score {
                    return Ok(());
                }
            } else {
                let point = (x, y).into();
                if id == 3 {
                    self.paddle = point;
                } else if id == 4 {
                    self.ball = point;
                } else {
                    self.arcade
                        .entry(point)
                        .and_modify(|v| *v = id)
                        .or_insert(id);
                }
                if first_score && id == 4 {
                    // arcade game is now running; we only calculate new input, if we received a new ball position
                    match self.paddle.x.cmp(&self.ball.x) {
                        Ordering::Greater => {
                            input = -1;
                            self.paddle.x -= 1;
                        }
                        Ordering::Equal => {
                            input = 0;
                        }
                        Ordering::Less => {
                            input = 1;
                            self.paddle.x += 1;
                        }
                    }
                    if !self.arcade.values().any(|v| *v == 2) {
                        // all blocks are destroyed, wait for one more score
                        last_score = true;
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_13.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 207);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, 10_247);

    Ok(())
}

#[cfg(test)]
mod tests {

    // no example to use for this challenge
    /*use super::*;

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_13_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_13 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }*/
}
