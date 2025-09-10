//!day_18.rs

use anyhow::Result;
use my_lib::{
    my_array::MyArray,
    my_compass::Compass,
    my_map_point::MapPoint,
    my_map_two_dim::{FilterFn, MyMap2D},
};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq)]
struct ChallengeInput<const X: usize, const Y: usize> {
    map: MyMap2D<char, X, Y>,
    collected_keys: MyArray<char, 26>,
    pos: MapPoint<X, Y>,
    steps: usize,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        let mut map: MyMap2D<char, X, Y> = MyMap2D::from(value);
        let (pos, _) = map.iter().find(|(_, v)| **v == '@').unwrap();
        map.set(pos, '.');

        ChallengeInput {
            map,
            collected_keys: MyArray::new(),
            pos,
            steps: 0,
        }
    }
}

impl<const X: usize, const Y: usize> PartialOrd for ChallengeInput<X, Y> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const X: usize, const Y: usize> Ord for ChallengeInput<X, Y> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.steps.cmp(&other.steps) {
            Ordering::Equal => match other.collected_keys.len().cmp(&self.collected_keys.len()) {
                Ordering::Equal => {
                    if self.collected_keys == other.collected_keys {
                        Ordering::Equal
                    } else {
                        self.collected_keys
                            .as_slice()
                            .iter()
                            .collect::<String>()
                            .cmp(&other.collected_keys.as_slice().iter().collect::<String>())
                    }
                }
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&self) -> usize {
        let mut sorted_queue: BTreeSet<ChallengeInput<X, Y>> = BTreeSet::new();
        sorted_queue.insert(*self);
        let keys: Vec<_> = self
            .map
            .iter()
            .filter_map(|(_, v)| v.is_ascii_lowercase().then_some(*v))
            .collect();
        dbg!(keys.len(), keys);
        while let Some(current) = sorted_queue.pop_first() {
            if !current.map.iter().any(|(_, v)| v.is_ascii_lowercase()) {
                // collected all keys
                println!("{:?}", current.collected_keys.as_slice());
                return current.steps;
            }
            let filter_fn: FilterFn<char, X, Y> = Box::new(
                |_point_of_next_cell: MapPoint<X, Y>,
                 value_of_next_cell: &char,
                 _orientation_of_next_cell: Compass,
                 _current_point: MapPoint<X, Y>,
                 _value_of_current_cell: &char,
                 _current_distance: usize| {
                    *value_of_next_cell == '.' || value_of_next_cell.is_ascii_lowercase()
                },
            );
            let keys: Vec<(MapPoint<X, Y>, &char, usize)> = current
                .map
                .iter_distance(current.pos, filter_fn)
                .filter(|(_, v, _)| v.is_ascii_lowercase())
                .collect();
            for (key_pos, key, distance) in keys {
                let mut next = current;
                next.collected_keys.push(*key);
                next.collected_keys.as_slice_mut().sort();
                next.pos = key_pos;
                next.steps += distance;
                next.map.set(key_pos, '.');
                let door = key.to_ascii_uppercase();
                if let Some(door_pos) = current
                    .map
                    .iter()
                    .find_map(|(p, v)| (*v == door).then_some(p))
                {
                    next.map.set(door_pos, '.');
                }
                sorted_queue.insert(next);
            }
        }
        0
    }
    fn solution_part_2(&self) -> u64 {
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_18.txt");
    let challenge = ChallengeInput::<81, 81>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_18 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_18 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_18_example_1.txt");
        let example = ChallengeInput::<24, 5>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 86);

        let result_part2 = example.solution_part_2();
        println!("result day_18 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY); };

        Ok(())
    }

    #[test]
    fn test_example_2_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_18_example_2.txt");
        let example = ChallengeInput::<24, 5>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 132);

        let result_part2 = example.solution_part_2();
        println!("result day_18 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }

    #[test]
    fn test_example_3_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_18_example_3.txt");
        let example = ChallengeInput::<1724, 9>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 136);

        let result_part2 = example.solution_part_2();
        println!("result day_18 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }

    #[test]
    fn test_example_4_day_18() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_18_example_4.txt");
        let example = ChallengeInput::<24, 6>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 81);

        let result_part2 = example.solution_part_2();
        println!("result day_18 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
