//!day_25.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

#[derive(Debug)]
struct Day25Data {
    keys: Vec<Vec<u8>>,
    locks: Vec<Vec<u8>>,
}

impl From<&str> for Day25Data {
    fn from(value: &str) -> Self {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut locks: Vec<Vec<u8>> = Vec::new();
        for schematic in value.split("\n\n").map(MyMap2D::<char, 5, 7>::from) {
            if schematic.get_row(0) == ['#'; 5] {
                let lock: Vec<u8> = [0, 1, 2, 3, 4_usize]
                    .into_iter()
                    .map(|c| {
                        schematic
                            .get_column(c)
                            .iter()
                            .skip(1)
                            .filter(|i| **i == '#')
                            .count() as u8
                    })
                    .collect();
                locks.push(lock);
            } else {
                let key: Vec<u8> = [0, 1, 2, 3, 4_usize]
                    .into_iter()
                    .map(|c| {
                        schematic
                            .get_column(c)
                            .iter()
                            .rev()
                            .skip(1)
                            .filter(|i| **i == '#')
                            .count() as u8
                    })
                    .collect();
                keys.push(key);
            }
        }
        Self { keys, locks }
    }
}

impl Day25Data {
    fn try_locks_and_keys(&self) -> usize {
        let mut fits = 0;
        for lock in self.locks.iter() {
            for key in self.keys.iter() {
                if lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6) {
                    fits += 1;
                }
            }
        }
        fits
    }
}

pub fn day_25() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_25.txt");
    let challenge = Day25Data::from(input);

    let result_part1 = challenge.try_locks_and_keys();
    println!("result day 25 part 1: {}", result_part1);
    assert_eq!(result_part1, 3_307);
    /*
    No part 2 on last day :)
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_25_example.txt");
        let challenge = Day25Data::from(input);

        let result_part1 = challenge.try_locks_and_keys();
        println!("result day 25 part 1: {}", result_part1);
        assert_eq!(result_part1, 3);
        /*
        No part 2 on last day :)
        */
        Ok(())
    }
}
