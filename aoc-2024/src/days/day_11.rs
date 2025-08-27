//!day_11.rs

use crate::utilities::digit_count;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Day11Data {
    challenge: Vec<u128>,
    blinks: u32,
}

impl From<&str> for Day11Data {
    fn from(value: &str) -> Self {
        Self {
            challenge: value
                .split_whitespace()
                .map(|n| n.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
            blinks: 0,
        }
    }
}

impl Day11Data {
    fn set_blinks(&mut self, blinks: u32) {
        self.blinks = blinks;
    }
    fn calc_stones(&self) -> u128 {
        let mut num_stones = 0;
        let mut cache: HashMap<(u128, u32), u128> = HashMap::new();
        for digit in self.challenge.iter() {
            num_stones += calc_stones_recursive(*digit, self.blinks, &mut cache);
        }
        num_stones
    }
}

fn calc_stones_recursive(
    digit: u128,
    num_blinks: u32,
    cache: &mut HashMap<(u128, u32), u128>,
) -> u128 {
    if num_blinks == 0 {
        return 1;
    }
    if let Some(stones) = cache.get(&(digit, num_blinks)) {
        return *stones;
    }
    let stones = if digit == 0 {
        calc_stones_recursive(1, num_blinks - 1, cache)
    } else {
        let n_digits = digit_count(digit);
        if n_digits & 1 == 1 {
            // odd number of digits
            calc_stones_recursive(digit * 2024, num_blinks - 1, cache)
        } else {
            // even number of digits
            let factor = 10_u128.pow(n_digits / 2);
            let left = digit / factor;
            let right = digit % factor;
            calc_stones_recursive(left, num_blinks - 1, cache)
                + calc_stones_recursive(right, num_blinks - 1, cache)
        }
    };
    cache.insert((digit, num_blinks), stones);
    stones
}

pub fn day_11() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_11.txt");
    let mut challenge = Day11Data::from(input);
    challenge.set_blinks(25);

    let result_part1 = challenge.calc_stones();
    println!("result day 11 part 1: {}", result_part1);
    assert_eq!(result_part1, 217_812);

    challenge.set_blinks(75);
    let result_part2 = challenge.calc_stones();
    println!("result day 11 part 2: {}", result_part2);
    assert_eq!(result_part2, 259_112_729_857_522);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_11_example.txt");
        let mut challenge = Day11Data::from(input);
        challenge.set_blinks(25);

        let result_part1 = challenge.calc_stones();
        println!("result day 11 part 1: {}", result_part1);
        assert_eq!(result_part1, 55_312);

        // no test result for part 2

        Ok(())
    }
}
