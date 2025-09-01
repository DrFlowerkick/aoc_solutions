//!day_23.rs

use anyhow::Result;
use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;

struct Cup {
    val: usize,
    link: RefCell<Option<Rc<Cup>>>,
}

impl Cup {
    fn next(&self) -> Rc<Cup> {
        self.link.borrow().as_ref().unwrap().clone()
    }
    fn relink(&self, link_target: Rc<Cup>) -> Rc<Cup> {
        *self.link.borrow_mut() = Some(link_target.clone());
        link_target
    }
}

struct CupList {
    current: Rc<Cup>,
    list: Vec<Rc<Cup>>,
    min_val: usize,
    max_val: usize,
}

impl From<&str> for CupList {
    fn from(value: &str) -> Self {
        let mut list: Vec<Rc<Cup>> = Vec::new();
        let mut min_val = usize::MAX;
        let mut max_val = 0;
        for (index, val) in value
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as usize)
            .enumerate()
        {
            min_val = min_val.min(val);
            max_val = max_val.max(val);
            let cup = Rc::new(Cup {
                val,
                link: RefCell::new(None),
            });
            list.push(cup.clone());
            if index > 0 {
                *list[index - 1].link.borrow_mut() = Some(cup.clone());
            }
        }
        // get current for start cup
        let current = list[0].clone();
        // close ring
        let last_index = list.len() - 1;
        *list[last_index].link.borrow_mut() = Some(current.clone());

        // sort list bei value of cup. Now "val - 1" gives us index to it's Rc<Cup>
        // this is the MAGIC with this approach comparing it to "day_23_slice_brute_force".
        // we get destination just by index instead of iterating through a Vec
        list.sort_by_key(|c| c.val);

        CupList {
            current,
            list,
            min_val,
            max_val,
        }
    }
}

impl CupList {
    fn extend_to_val(&mut self, new_max: usize) {
        if new_max <= self.max_val {
            return;
        }
        // get "last" element -> element which points to current
        let current_val = self.current.val;
        let mut cup = self.current.clone();
        while cup.link.borrow().as_ref().unwrap().val != current_val {
            cup = cup.next();
        }
        let mut last_val = cup.val;
        // extend ring and list
        for val in self.max_val + 1..=new_max {
            let cup = Rc::new(Cup {
                val,
                link: RefCell::new(None),
            });
            self.list.push(cup.clone());
            *self.list[last_val - 1].link.borrow_mut() = Some(cup.clone());
            last_val = val;
        }
        // close ring
        *self.list[last_val - 1].link.borrow_mut() = Some(self.current.clone());
        self.max_val = new_max;
    }
    fn get_pick_up(&self) -> Rc<Cup> {
        self.current.next()
    }
    fn get_end_of_pick_up(&self) -> Rc<Cup> {
        self.current.next().next().next()
    }
    fn get_destination(&self) -> Rc<Cup> {
        let pick_up = self.get_pick_up();
        let second_pick_up = pick_up.next();
        let third_pick_up = second_pick_up.next();
        let mut destination_val = self.current.val;
        loop {
            destination_val = if destination_val == self.min_val {
                self.max_val
            } else {
                destination_val - 1
            };
            if destination_val != pick_up.val
                && destination_val != second_pick_up.val
                && destination_val != third_pick_up.val
            {
                break;
            }
        }
        self.list[destination_val - 1].clone()
    }
    fn set_next_current(&mut self) {
        self.current = self.current.next();
    }
    fn get_values_in_ring(&self, start_value: usize) -> Vec<usize> {
        if start_value < self.min_val || start_value > self.max_val {
            return vec![];
        }
        let mut values_in_ring: Vec<usize> = Vec::with_capacity(self.list.len());
        values_in_ring.push(start_value);
        let mut cup = self.list[start_value - 1].clone();
        while cup.link.borrow().as_ref().unwrap().val != start_value {
            cup = cup.next();
            values_in_ring.push(cup.val);
        }

        values_in_ring
    }
    fn get_cup_by_val(&self, val: usize) -> Rc<Cup> {
        self.list[val - 1].clone()
    }
}

struct ChallengeInput {
    cups: CupList,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            cups: CupList::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> String {
        self.do_moves(100);

        let mut cups = String::new();
        self.cups
            .get_values_in_ring(1)
            .iter()
            .skip(1)
            .for_each(|c| {
                write!(&mut cups, "{c}").unwrap();
            });
        cups
    }
    fn solution_part_2(&mut self) -> usize {
        self.cups.extend_to_val(1_000_000);
        self.do_moves(10_000_000);

        let cup_1 = self.cups.get_cup_by_val(1);
        let next_val = cup_1.next().val;
        let next_next_val = cup_1.next().next().val;
        next_val * next_next_val
    }
    fn do_moves(&mut self, moves: usize) {
        for _ in 0..moves {
            let pick_up = self.cups.get_pick_up();
            let end_of_pick_up = self.cups.get_end_of_pick_up();
            let destination = self.cups.get_destination();
            // link current to cup next after end of pick up
            self.cups.current.relink(end_of_pick_up.next());
            // link end of pick up to cup next after destination
            end_of_pick_up.relink(destination.next());
            // link destination to pick up: ring is established again!
            destination.relink(pick_up);
            // finally set current to next in ring
            self.cups.set_next_current();
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_23.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, "65432978");

    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 287_230_227_046);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_23_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1: {result_part1}");
        assert_eq!(result_part1, "67384529");

        let input = include_str!("../../../../aoc_input/aoc-2020/day_23_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        assert_eq!(result_part2, 149_245_887_792);

        Ok(())
    }
}
