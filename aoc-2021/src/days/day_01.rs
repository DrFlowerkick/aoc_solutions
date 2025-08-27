//!day_01.rs

use anyhow::Result;

fn count_increases(input: &str) -> usize {
    let previous = i64::MAX;
    input
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .fold((previous, 0), |(prev, mut count), current| {
            if current > prev {
                count += 1;
            }
            (current, count)
        })
        .1
}

fn count_increases_sliding_window(input: &str, window_size: usize) -> usize {
    let inputs: Vec<i64> = input
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .collect();
    let mut previous = i64::MAX;
    let mut count = 0;
    for i in 0..=inputs.len() - window_size {
        let current: i64 = inputs[i..i + window_size].iter().sum();
        if current > previous {
            count += 1;
        }
        previous = current;
    }
    count
}

pub fn day_01() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_01.txt");

    let result_part1 = count_increases(input);
    println!("result day 01 part 1: {result_part1}");
    assert_eq!(result_part1, 1_527);

    let result_part2 = count_increases_sliding_window(input, 3);
    println!("result day 01 part 2: {result_part2}");
    assert_eq!(result_part2, 1_575);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_01_example.txt");

        let result_part1 = count_increases(input);
        println!("result day 01 part 1: {result_part1}");
        assert_eq!(result_part1, 7);

        let result_part2 = count_increases_sliding_window(input, 3);
        println!("result day 01 part 2: {result_part2}");
        assert_eq!(result_part2, 5);

        Ok(())
    }
}
