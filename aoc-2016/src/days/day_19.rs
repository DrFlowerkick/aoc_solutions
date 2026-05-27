//!day_19.rs

use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

struct Elf {
    value: u64,
    next: RefCell<Option<Rc<Elf>>>,
}

impl Elf {
    fn new(value: u64) -> Rc<Elf> {
        let new = Rc::new(Elf {
            value,
            next: RefCell::new(None),
        });
        new.relink_next(new.clone());
        new
    }
    fn next(&self) -> Rc<Elf> {
        self.next.borrow().as_ref().unwrap().clone()
    }
    fn n_next(&self, mut steps: u64) -> Rc<Elf> {
        if steps == 0 {
            panic!("steps must be greater zero");
        }
        let mut next = self.next();
        while steps > 1 {
            next = next.next();
            steps -= 1;
        }
        next
    }
    fn relink_next(&self, next: Rc<Elf>) {
        *self.next.borrow_mut() = Some(next.clone());
    }
    fn insert(&self, value: u64) -> Rc<Elf> {
        let new = Self::new(value);
        let next = self.next();
        new.relink_next(next);
        self.relink_next(new.clone());
        new
    }
    fn skip_next(&self) -> Rc<Elf> {
        let next = self.next();
        if next.value == self.value {
            return next;
        }
        let next = next.next();
        self.relink_next(next.clone());
        next
    }
}

struct ChallengeInput {
    num_elves: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            num_elves: value.parse().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let first = Elf::new(1);
        let mut current = first.clone();
        while current.value < self.num_elves {
            current = current.insert(current.value + 1);
        }
        current = first;
        loop {
            let next = current.skip_next();
            if current.value == next.value {
                return current.value;
            }
            current = next;
        }
    }
    fn solution_part_2(&self) -> u64 {
        let first = Elf::new(1);
        let mut current = first.clone();
        while current.value < self.num_elves {
            current = current.insert(current.value + 1);
        }
        // first elv to skip is at half the number of elves
        // therefore we start to skip elves at half the number of elves - 1
        let steps = self.num_elves / 2 - 1;
        let mut before_skip = first.n_next(steps);
        let mut count = self.num_elves;
        while count > 1 {
            let maybe_next_before_skip = before_skip.skip_next();
            if count & 1 == 1 {
                // we jump to elv after skipped elv, if the count of elves before
                // skipping is odd --> our skip position runs around the circle
                // in sync with the elv which will commit the theft.
                before_skip = maybe_next_before_skip;
            }
            count -= 1;
        }
        before_skip.value
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_19.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, 1_815_603);

    let result_part2 = challenge.solution_part_2();
    println!("result day_19 part 2: {result_part2}");
    assert_eq!(result_part2, 1_410_630);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_19() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_19_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_19 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        let result_part2 = example.solution_part_2();
        println!("result day_19 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
