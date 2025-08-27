//!day_07.rs

use crate::utilities::digit_count;
use anyhow::Result;

#[derive(Debug)]
struct TestData {
    result: u128,
    items: Vec<u128>,
}

impl From<&str> for TestData {
    fn from(value: &str) -> Self {
        let (result, items) = value.split_once(':').unwrap();
        Self {
            result: result.parse().unwrap(),
            items: items
                .split_whitespace()
                .filter_map(|i| i.parse::<u128>().ok())
                .collect(),
        }
    }
}

impl TestData {
    fn test_data(&self, concatenation: bool) -> bool {
        assert!(!self.items.is_empty());
        self.test_data_slice(self.items[0], &self.items[1..], concatenation)
    }
    fn test_data_slice(&self, current_result: u128, slice: &[u128], concatenation: bool) -> bool {
        if slice.is_empty() {
            return self.result == current_result;
        }
        let add_result = current_result + slice[0];
        if self.test_data_slice(add_result, &slice[1..], concatenation) {
            return true;
        }
        let mul_result = current_result * slice[0];
        if self.test_data_slice(mul_result, &slice[1..], concatenation) {
            return true;
        }
        if concatenation {
            // example of concatenation
            // current_result = 123, slice[0] = 45, concatenation of both is 12345
            let num_digits = digit_count(slice[0]);
            let con_result = current_result * 10_u128.pow(num_digits) + slice[0];
            self.test_data_slice(con_result, &slice[1..], concatenation)
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Day07Data {
    samples: Vec<TestData>,
}

impl From<&str> for Day07Data {
    fn from(value: &str) -> Self {
        Self {
            samples: value.lines().map(TestData::from).collect(),
        }
    }
}

impl Day07Data {
    fn test_samples(&self) -> u128 {
        self.samples
            .iter()
            .filter(|td| td.test_data(false))
            .map(|td| td.result)
            .sum()
    }
    fn test_samples_with_concatenation(&self) -> u128 {
        self.samples
            .iter()
            .filter(|td| td.test_data(true))
            .map(|td| td.result)
            .sum()
    }
}

pub fn day_07() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_07.txt");
    let challenge = Day07Data::from(input);

    let result_part1 = challenge.test_samples();
    println!("result day 07 part 1: {}", result_part1);
    assert_eq!(result_part1, 20_665_830_408_335);

    let result_part2 = challenge.test_samples_with_concatenation();
    println!("result day 07 part 2: {}", result_part2);
    assert_eq!(result_part2, 354_060_705_047_464);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_07_example.txt");
        let challenge = Day07Data::from(input);

        let result_part1 = challenge.test_samples();
        println!("result day 07 part 1: {}", result_part1);
        assert_eq!(result_part1, 3_749);

        let result_part2 = challenge.test_samples_with_concatenation();
        println!("result day 07 part 2: {}", result_part2);
        assert_eq!(result_part2, 11_387);

        Ok(())
    }
}
