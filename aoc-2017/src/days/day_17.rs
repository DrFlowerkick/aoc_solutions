//!day_17.rs

use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

struct Spinlock {
    value: u64,
    next: RefCell<Option<Rc<Spinlock>>>,
}

impl Spinlock {
    fn new(value: u64) -> Rc<Spinlock> {
        let new = Rc::new(Spinlock {
            value,
            next: RefCell::new(None),
        });
        new.relink_next(new.clone());
        new
    }
    fn next(&self) -> Rc<Spinlock> {
        self.next.borrow().as_ref().unwrap().clone()
    }
    fn n_next(&self, steps: u64) -> Rc<Spinlock> {
        if steps == 0 {
            panic!("steps must be greater zero");
        }
        let next = self.next();
        if steps == 1 {
            return next;
        }
        next.n_next(steps - 1)
    }
    fn relink_next(&self, next: Rc<Spinlock>) {
        *self.next.borrow_mut() = Some(next.clone());
    }
    fn insert(&self, value: u64) -> Rc<Spinlock> {
        let new = Self::new(value);
        let next = self.next();
        new.relink_next(next);
        self.relink_next(new.clone());
        new
    }
}

struct ChallengeInput {
    steps: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            steps: value.parse().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut value = 0;
        let mut spin_lock = Spinlock::new(value);
        while value < 2017 {
            value += 1;
            spin_lock = spin_lock.n_next(self.steps);
            spin_lock = spin_lock.insert(value);
        }
        spin_lock.next().value
    }
    fn solution_part_2(&self) -> u64 {
        let mut current_pos = 0;
        let mut value_after_zero = 0;

        // Instead of Spinlock we use a mathematic solution. This is possible,
        // because element of value 0 always stays at index 0 of list. Therefore
        // we only have to check if a new value is added at pos 1.
        for n in 1..=50_000_000 {
            // each new value increases the length of list by 1.
            // therefore we calculate the new pos by adding the steps to the current
            // pos (using mod n to account for cycling of the end of the list) and
            // than just add 1.
            current_pos = (current_pos + self.steps).rem_euclid(n) + 1;

            // if current pos is equal to 1, we have a new value for the element after zero
            if current_pos == 1 {
                value_after_zero = n;
            }
        }
        value_after_zero
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_17.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, 1_311);

    let result_part2 = challenge.solution_part_2();
    println!("result day_17 part 2: {result_part2}");
    assert_eq!(result_part2, 39_170_601);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_17() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_17_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 638);

        Ok(())
    }
}
