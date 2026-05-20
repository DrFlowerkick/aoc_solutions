//!day_11.rs

use anyhow::Result;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
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
    fn is_pair(&self, other: &Item) -> bool {
        match (self, other) {
            (Item::Generator(g), Item::Microchip(m)) | (Item::Microchip(m), Item::Generator(g)) => {
                g == m
            }
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Floor<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> From<&'a str> for Floor<'a> {
    fn from(value: &'a str) -> Self {
        let mut items: Vec<Item<'a>> = value
            .split(" a ")
            .skip(1)
            .flat_map(|w| w.split_whitespace().next())
            .map(Item::from)
            .collect();
        items.sort();
        Floor { items }
    }
}

impl<'a> From<&Item<'a>> for Floor<'a> {
    fn from(value: &Item<'a>) -> Self {
        Floor {
            items: vec![*value],
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
                floor.items.push(*item_b);
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
        self.items.sort();
        self.is_valid()
    }
    fn count_pairs(&self) -> usize {
        let mut count = 0;
        for (i, item_a) in self.items.iter().enumerate() {
            for item_b in self.items.iter().skip(i + 1) {
                if item_a.is_pair(item_b) {
                    count += 1;
                }
            }
        }
        count
    }
    fn count_generators(&self) -> usize {
        self.items
            .iter()
            .filter(|i| matches!(i, Item::Generator(_)))
            .filter(|g| self.items.iter().all(|i| !i.is_pair(g)))
            .count()
    }
    fn count_microchips(&self) -> usize {
        self.items
            .iter()
            .filter(|i| matches!(i, Item::Microchip(_)))
            .filter(|m| self.items.iter().all(|i| !i.is_pair(m)))
            .count()
    }
}

// to prevent redundant moves we normalize the representation of floors
// by counting for each floor the number of pairs, solo generators, and solo microchips.
// This removes the identity of each item, but keeps the overall distribution of items.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct NormalizedFloors {
    pairs: Vec<usize>,
    solo_generators: Vec<usize>,
    solo_microchips: Vec<usize>,
    pos: usize,
}

impl<'a> From<&ChallengeInput<'a>> for NormalizedFloors {
    fn from(value: &ChallengeInput) -> Self {
        NormalizedFloors {
            pairs: value
                .floors
                .iter()
                .map(|floor| floor.count_pairs())
                .collect(),
            solo_generators: value
                .floors
                .iter()
                .map(|floor| floor.count_generators())
                .collect(),
            solo_microchips: value
                .floors
                .iter()
                .map(|floor| floor.count_microchips())
                .collect(),
            pos: value.pos,
        }
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

impl<'a> Ord for ChallengeInput<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.heuristic().cmp(&other.heuristic()) {
            Ordering::Equal => match self.floors.cmp(&other.floors) {
                Ordering::Equal => self.pos.cmp(&other.pos),
                cmp => cmp,
            },
            cmp => cmp,
        }
    }
}

impl<'a> PartialOrd for ChallengeInput<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> ChallengeInput<'a> {
    fn heuristic(&self) -> usize {
        self.floors
            .iter()
            .enumerate()
            .map(|(i, f)| (i + 1) * f.items.len())
            .sum()
    }
    fn solution_part_1(&self) -> i64 {
        let mut seen: HashMap<NormalizedFloors, i64> = HashMap::new();
        let mut sorted_queue: BTreeSet<(ChallengeInput, i64)> = BTreeSet::new();
        sorted_queue.insert((self.clone(), 0));
        let mut min_steps = i64::MAX;
        // we use negative steps, because in case of identical heuristic value we want to continue with least steps
        while let Some((state, step)) = sorted_queue.pop_last() {
            // skip to many steps
            if -step >= min_steps {
                continue;
            }

            // check if state already existed
            let normalize = NormalizedFloors::from(&state);
            if let Some(seen_steps) = seen.get(&normalize)
                && -step >= *seen_steps
            {
                // this state already happened with less steps
                continue;
            }
            // insert normalized state into seen cache
            seen.insert(normalize, -step);

            // check for final state
            if state.pos == 3
                && state.floors[0].is_empty()
                && state.floors[1].is_empty()
                && state.floors[2].is_empty()
            {
                min_steps = min_steps.min(-step);
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
                    if down.floors[down.pos].push_elevator(&elevator) {
                        sorted_queue.insert((down, step - 1));
                    }
                }
                if state.pos < 3 {
                    // move one up
                    let mut up = enter_elevator.clone();
                    up.pos += 1;
                    if up.floors[up.pos].push_elevator(&elevator) {
                        sorted_queue.insert((up, step - 1));
                    }
                }
            }
        }
        min_steps
    }
    fn solution_part_2(&mut self) -> i64 {
        self.floors[0].items.push(Item::Generator("elerium"));
        self.floors[0].items.push(Item::Microchip("elerium"));
        self.floors[0].items.push(Item::Generator("dilithium"));
        self.floors[0].items.push(Item::Microchip("dilithium"));
        self.solution_part_1()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, 33);

    let mut challenge = ChallengeInput::from(input);
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

        Ok(())
    }
}
