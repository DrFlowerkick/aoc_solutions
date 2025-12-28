//!day_09.rs

use anyhow::Result;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Marble {
    value: u64,
    previous: RefCell<Option<Rc<Marble>>>,
    next: RefCell<Option<Rc<Marble>>>,
}

impl Marble {
    fn new(value: u64) -> Rc<Marble> {
        let new = Rc::new(Marble {
            value,
            previous: RefCell::new(None),
            next: RefCell::new(None),
        });
        new.relink_next(new.clone());
        new.relink_previous(new.clone());
        new
    }
    fn next(&self) -> Rc<Marble> {
        self.next.borrow().as_ref().unwrap().clone()
    }
    fn previous(&self) -> Rc<Marble> {
        self.previous.borrow().as_ref().unwrap().clone()
    }
    fn relink_next(&self, next: Rc<Marble>) {
        *self.next.borrow_mut() = Some(next.clone());
    }
    fn relink_previous(&self, previous: Rc<Marble>) {
        *self.previous.borrow_mut() = Some(previous.clone());
    }
    fn remove(&self) -> Rc<Marble> {
        let previous = self.previous();
        let next = self.next();
        previous.relink_next(next.clone());
        next.relink_previous(previous);
        next.clone()
    }
}

struct ChallengeInput {
    players: u64,
    last_marble: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let caps = re.captures(value).unwrap();
        let players = caps[1].parse().unwrap();
        let last_marble = caps[2].parse().unwrap();
        ChallengeInput {
            players,
            last_marble,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self, part_1_only: bool) -> (u64, u64) {
        let mut part_1 = 0;
        let factor = if part_1_only { 1 } else { 100 };
        let mut player: u64 = 0;
        let mut scores: HashMap<u64, u64> = HashMap::new();
        let mut value = 0;
        let mut marble = Marble::new(value);
        while value < self.last_marble * factor {
            value += 1;
            if value.is_multiple_of(23) {
                for _ in 0..7 {
                    marble = marble.previous();
                }
                let score = value + marble.value;
                scores
                    .entry(player)
                    .and_modify(|v| *v += score)
                    .or_insert(score);
                marble = marble.remove();
            } else {
                let left = marble.next();
                let right = left.next();
                let new = Marble::new(value);
                left.relink_next(new.clone());
                new.relink_previous(left.clone());
                new.relink_next(right.clone());
                right.relink_previous(new.clone());
                marble = new;
            }
            // next player
            player += 1;
            if player.is_multiple_of(self.players) {
                player = 0;
            }
            // part 1
            if value == self.last_marble {
                part_1 = *scores.values().max().unwrap();
            }
        }
        (part_1, *scores.values().max().unwrap())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(false);
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 393_229);

    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 3_273_405_195);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_09() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_09_example.txt");

        let solutions = [32, 8_317, 146_373, 2_764, 54_718, 37_305];
        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let (result_part1, _) = example.solution_part_1_and_2(true);
            println!("result day_09 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }
}
