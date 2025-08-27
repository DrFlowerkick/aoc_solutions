//!day_03.rs

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
struct Day03Data {
    factors: Vec<(i128, i128)>,
}

impl From<&str> for Day03Data {
    fn from(value: &str) -> Self {
        // Regex for mul(XXX,YYY)
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to create regex");
        Self {
            factors: re
                .captures_iter(value)
                .filter_map(|cap| cap.get(1).and_then(|c1| cap.get(2).map(|c2| (c1, c2))))
                .filter_map(|(c1, c2)| {
                    c1.as_str()
                        .parse::<i128>()
                        .ok()
                        .and_then(|n1| c2.as_str().parse::<i128>().ok().map(|n2| (n1, n2)))
                })
                .collect(),
        }
    }
}

impl Day03Data {
    fn add_up_multiplied_factors(&self) -> i128 {
        self.factors.iter().map(|(n1, n2)| n1 * n2).sum()
    }
}

#[derive(Debug)]
struct Day03DataWithFlags {
    factors: Vec<(i128, i128)>,
}

impl From<&str> for Day03DataWithFlags {
    fn from(value: &str) -> Self {
        // Regex for mul(XXX,YYY) or do() or don't()
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
            .expect("Failed to create regex");
        let mut factors: Vec<(i128, i128)> = Vec::new();
        let mut enabled = true;
        for cap in re.captures_iter(value) {
            match cap.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let n1 = cap.get(1).unwrap().as_str().parse::<i128>().unwrap();
                        let n2 = cap.get(2).unwrap().as_str().parse::<i128>().unwrap();
                        factors.push((n1, n2));
                    }
                }
            }
        }

        Self { factors }
    }
}

impl Day03DataWithFlags {
    fn add_up_multiplied_factors(&self) -> i128 {
        self.factors.iter().map(|(n1, n2)| n1 * n2).sum()
    }
}

pub fn day_03() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_03.txt");
    let challenge = Day03Data::from(input);

    let result_part1 = challenge.add_up_multiplied_factors();
    println!("result day 03 part 1: {}", result_part1);
    assert_eq!(result_part1, 171_183_089);

    let challenge = Day03DataWithFlags::from(input);
    let result_part2 = challenge.add_up_multiplied_factors();
    println!("result day 03 part 2: {}", result_part2);
    assert_eq!(result_part2, 63_866_497);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_03_example_01.txt");
        let challenge = Day03Data::from(input);

        let result_part1 = challenge.add_up_multiplied_factors();
        println!("result day 03 part 1: {}", result_part1);
        assert_eq!(result_part1, 161);

        let input = include_str!("../../../../aoc_input/aoc-2024/day_03_example_02.txt");
        let challenge = Day03DataWithFlags::from(input);
        let result_part2 = challenge.add_up_multiplied_factors();
        println!("result day 03 part 2: {}", result_part2);
        assert_eq!(result_part2, 48);

        Ok(())
    }
}
