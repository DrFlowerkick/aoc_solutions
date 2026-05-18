//!day_11.rs

use anyhow::Result;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Item<'a> {
    Generator(&'a str),
    Microchip(&'a str),
}

impl<'a> From<&'a str> for Item<'a> {
    fn from(value: &'a str) -> Self {
        if let Some(microchip) = value.strip_suffix("-compatible") {
            Item::Microchip(microchip)
        } else {
            Item::Generator(value)
        }
    }
}

impl<'a> Item<'a> {
    fn is_generator(&self) -> bool {
        matches!(self, Item::Generator(_))
    }
    fn get_microchip_name(&self) -> Option<&str> {
        if let Item::Microchip(mc) = self {
            Some(mc)
        } else {
            None
        }
    }
    fn match_generator(&self, mc: &str) -> bool {
        if let Item::Generator(g) = self
            && *g == mc
        {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Floor<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> From<&'a str> for Floor<'a> {
    fn from(value: &'a str) -> Self {
        Floor {
            items: value
                .split(" a ")
                .skip(1)
                .flat_map(|w| w.split_whitespace().next())
                .map(Item::from)
                .collect(),
        }
    }
}

impl<'a> From<&Item<'a>> for Floor<'a> {
    fn from(value: &Item<'a>) -> Self {
        Floor {
            items: vec![value.clone()],
        }
    }
}

impl<'a> Floor<'a> {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn is_valid(&self) -> bool {
        if self.items.iter().any(|i| i.is_generator()) {
            self.items
                .iter()
                .filter_map(|i| i.get_microchip_name())
                .all(|mc| self.items.iter().any(|g| g.match_generator(mc)))
        } else {
            true
        }
    }
    fn possible_elevators(&self) -> Vec<Floor<'a>> {
        let mut elevators: Vec<Floor> = self.items.iter().map(Floor::from).collect();
        for (i, item_a) in self.items.iter().enumerate() {
            for item_b in self.items.iter().skip(i + 1) {
                let mut floor = Floor::from(item_a);
                floor.items.push(item_b.clone());
                if floor.is_valid() {
                    elevators.push(floor);
                }
            }
        }
        elevators
    }
    fn pop_elevator(&mut self, elevator: &Floor<'a>) -> bool {
        for item in elevator.items.iter() {
            if let Some(pos) = self.items.iter().position(|i| i == item) {
                self.items.remove(pos);
            }
        }
        self.is_valid()
    }
    fn push_elevator(&mut self, elevator: &Floor<'a>) -> bool {
        self.items.extend_from_slice(&elevator.items);
        self.is_valid()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct ChallengeInput<'a> {
    floors: [Floor<'a>; 4],
    pos: usize,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let mut floor_iter = value.lines().map(Floor::from);
        ChallengeInput {
            floors: [
                floor_iter.next().unwrap(),
                floor_iter.next().unwrap(),
                floor_iter.next().unwrap(),
                floor_iter.next().unwrap(),
            ],
            pos: 0,
        }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u64 {
        let mut seen: HashSet<ChallengeInput> = HashSet::new();
        let mut queue: VecDeque<(ChallengeInput, u64)> = VecDeque::new();
        queue.push_back((self.clone(), 0));
        while let Some((state, step)) = queue.pop_front() {
            if seen.insert(state.clone()) {
                // check for final state
                if state.floors[0].is_empty()
                    && state.floors[1].is_empty()
                    && state.floors[2].is_empty()
                {
                    return step;
                }
                // move items
                for elevator in state.floors[state.pos].possible_elevators() {
                    // check if remaining items are valid
                    let mut enter_elevator = state.clone();
                    if !enter_elevator.floors[enter_elevator.pos].pop_elevator(&elevator) {
                        // invalid remaining items, skip elevator
                        continue;
                    }

                    if state.pos > 0 {
                        // move one down
                        let mut down = enter_elevator.clone();
                        down.pos -= 1;
                        if down.floors[down.pos].push_elevator(&elevator) && !seen.contains(&down) {
                            queue.push_back((down, step + 1));
                        }
                    }
                    if state.pos < 3 {
                        // move one up
                        let mut up = enter_elevator.clone();
                        up.pos += 1;
                        if up.floors[up.pos].push_elevator(&elevator) && !seen.contains(&up) {
                            queue.push_back((up, step + 1));
                        }
                    }
                }
            }
        }
        0
    }
    fn solution_part_2(&self) -> u64 {
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_11 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_11_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_11 part 1: {result_part1}");
        assert_eq!(result_part1, 11);

        let result_part2 = example.solution_part_2();
        println!("result day_11 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
