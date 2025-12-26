//!day_05.rs

use anyhow::Result;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

struct Unit {
    unit_type: char,
    polarity: bool,
    previous: RefCell<Option<Rc<Unit>>>,
    next: RefCell<Option<Rc<Unit>>>,
}

impl Unit {
    fn new(unit_type: char, previous: Option<Rc<Unit>>) -> Rc<Unit> {
        let new = Rc::new(Unit {
            unit_type: unit_type.to_ascii_lowercase(),
            polarity: unit_type.is_ascii_uppercase(),
            previous: RefCell::new(previous.clone()),
            next: RefCell::new(None),
        });
        if let Some(ref previous) = previous {
            previous.relink(new.clone());
        }
        new
    }
    fn next(&self) -> Option<Rc<Unit>> {
        self.next.borrow().as_ref().cloned()
    }
    fn previous(&self) -> Option<Rc<Unit>> {
        self.previous.borrow().as_ref().cloned()
    }
    fn relink(&self, link_target: Rc<Unit>) -> Rc<Unit> {
        *self.next.borrow_mut() = Some(link_target.clone());
        link_target
    }
    fn unlink(&self) {
        *self.next.borrow_mut() = None;
    }
    fn reacts_with(&self, other: Rc<Unit>) -> bool {
        self.unit_type == other.unit_type && self.polarity != other.polarity
    }
    fn count(&self) -> u64 {
        if let Some(next) = self.next() {
            1 + next.count()
        } else {
            1
        }
    }
}

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(input: &'a str) -> Self {
        ChallengeInput { input }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u64 {
        self.process_polymer(None)
    }
    fn solution_part_2(&self) -> u64 {
        let unique_types: HashSet<char> =
            self.input.chars().map(|c| c.to_ascii_lowercase()).collect();
        let mut min = u64::MAX;
        for f in unique_types {
            min = min.min(self.process_polymer(Some(f)));
        }
        min
    }
    fn process_polymer(&self, filter: Option<char>) -> u64 {
        let mut first = None::<Rc<Unit>>;
        let mut current = None::<Rc<Unit>>;
        for c in self.input.chars().filter(|c| {
            if let Some(f) = filter {
                c.to_ascii_lowercase() != f
            } else {
                true
            }
        }) {
            let next = Unit::new(c, current.clone());
            if first.is_none() {
                first = Some(next.clone());
            }
            if let Some(cur) = current {
                if cur.reacts_with(next.clone()) {
                    if let Some(previous) = cur.previous() {
                        previous.unlink();
                        current = Some(previous);
                    } else {
                        current = None;
                        first = None;
                    }
                } else {
                    current = Some(next);
                }
            } else {
                current = Some(next);
            }
        }
        first.unwrap().count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_05.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 11_814);

    let result_part2 = challenge.solution_part_2();
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 4_282);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_05() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_05_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, 10);

        let result_part2 = example.solution_part_2();
        println!("result day_05 part 2: {result_part2}");
        assert_eq!(result_part2, 4);

        Ok(())
    }
}
