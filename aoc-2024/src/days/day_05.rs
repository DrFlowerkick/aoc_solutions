//!day_05.rs

use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Day05Data {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl From<&str> for Day05Data {
    fn from(value: &str) -> Self {
        let (rules_str, updates) = value.split_once("\n\n").unwrap();
        let mut rules: HashMap<u32, Vec<u32>> = HashMap::with_capacity(100);
        for line in rules_str.lines() {
            let (left, right) = line.split_once('|').unwrap();
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();
            rules
                .entry(left)
                .and_modify(|v| v.push(right))
                .or_insert(vec![right]);
        }
        let updates: Vec<Vec<u32>> = updates
            .lines()
            .map(|line| {
                line.split(',')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect()
            })
            .collect();

        Self { rules, updates }
    }
}

impl Day05Data {
    fn add_middle_numbers_of_correct_order_updates(&self) -> u32 {
        let mut sum_middle_numbers = 0;
        for update in self.updates.iter() {
            // len of update should be odd
            assert!(update.len() & 1 == 1);
            if !update.is_sorted_by(|a, b| self.less(a, b)) {
                // skip current update and move on to next one
                continue;
            }
            sum_middle_numbers += update[update.len() / 2];
        }
        sum_middle_numbers
    }

    fn add_middle_numbers_of_incorrect_order_updates(&self) -> u32 {
        let mut sum_middle_numbers = 0;
        for update in self.updates.iter() {
            // len of update should be odd
            assert!(update.len() & 1 == 1);
            if update.is_sorted_by(|a, b| self.less(a, b)) {
                // skip current update and move on to next one
                continue;
            }
            // first sort update
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| self.compare(a, b));
            sum_middle_numbers += sorted[sorted.len() / 2];
        }
        sum_middle_numbers
    }

    fn less(&self, left: &u32, right: &u32) -> bool {
        match self.compare(left, right) {
            Ordering::Equal => panic!("update should not contain equal numbers."),
            Ordering::Less => true,
            Ordering::Greater => false,
        }
    }

    fn compare(&self, left: &u32, right: &u32) -> Ordering {
        if left == right {
            return Ordering::Equal;
        }
        match self.rules.get(left) {
            Some(left_order_items) => {
                if left_order_items.iter().any(|loi| loi == right) {
                    // Less -> left is left of right
                    Ordering::Less
                } else {
                    // Greater -> left should be right of right
                    Ordering::Greater
                }
            }
            None => Ordering::Greater, // Greater -> left should be right of right
        }
    }
}

pub fn day_05() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_05.txt");
    let challenge = Day05Data::from(input);

    let result_part1 = challenge.add_middle_numbers_of_correct_order_updates();
    println!("result day 05 part 1: {}", result_part1);
    assert_eq!(result_part1, 5_732);

    let result_part2 = challenge.add_middle_numbers_of_incorrect_order_updates();
    println!("result day 05 part 2: {}", result_part2);
    assert_eq!(result_part2, 4_716);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_05_example.txt");
        let challenge = Day05Data::from(input);

        let result_part1 = challenge.add_middle_numbers_of_correct_order_updates();
        println!("result day 05 part 1: {}", result_part1);
        assert_eq!(result_part1, 143);

        let result_part2 = challenge.add_middle_numbers_of_incorrect_order_updates();
        println!("result day 05 part 2: {}", result_part2);
        assert_eq!(result_part2, 123);

        Ok(())
    }
}
