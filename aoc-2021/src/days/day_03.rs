//!day_03.rs

use anyhow::Result;

fn binary_diagnostic_power_consumption(input: &str) -> u64 {
    let mut columns: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if i >= columns.len() {
                columns.push(Vec::new());
            }
            columns[i].push(c == '1');
        }
    }
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (index, col) in columns.iter().enumerate() {
        let ones = col.iter().filter(|c| **c).count();
        let zeros = col.len() - ones;
        match ones.cmp(&zeros) {
            std::cmp::Ordering::Greater => {
                gamma_rate += 1 << (columns.len() - index - 1);
            }
            std::cmp::Ordering::Less => {
                epsilon_rate += 1 << (columns.len() - index - 1);
            }
            _ => println!("What should we do in equal case?"),
        }
    }
    gamma_rate * epsilon_rate
}

fn binary_diagnostic_life_support_rating(input: &str) -> u64 {
    let mut columns: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if i >= columns.len() {
                columns.push(Vec::new());
            }
            columns[i].push(c == '1');
        }
    }

    let oxygen_generator_rate = filter_for_rate(&columns, true);
    let co2_scrubber_rate = filter_for_rate(&columns, false);

    oxygen_generator_rate * co2_scrubber_rate
}

fn filter_for_rate(columns: &[Vec<bool>], check_most_common: bool) -> u64 {
    let mut filter = vec![true; columns[0].len()];
    let mut rate = 0;
    for col in columns.iter() {
        let ones = col
            .iter()
            .zip(filter.iter())
            .filter(|(c, f)| **c && **f)
            .count();
        let zeros = filter.iter().filter(|f| **f).count() - ones;
        let bits_to_keep = match ones.cmp(&zeros) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => check_most_common,
            std::cmp::Ordering::Less => !check_most_common,
        };
        filter = filter
            .iter()
            .zip(col.iter())
            .map(|(f, c)| *f && (*c == bits_to_keep))
            .collect();
        if filter.iter().filter(|f| **f).count() == 1 {
            let row = filter.iter().position(|f| *f).unwrap();
            for (index, col) in columns.iter().enumerate() {
                if col[row] {
                    rate += 1 << (columns.len() - index - 1);
                }
            }
        }
    }
    rate
}

pub fn day_03() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_03.txt");

    let result_part1 = binary_diagnostic_power_consumption(input);
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 1_092_896);

    let result_part2 = binary_diagnostic_life_support_rating(input);
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 4_672_151);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_03_example.txt");

        let result_part1 = binary_diagnostic_power_consumption(input);
        println!("result day_03 part 1: {result_part1}");
        assert_eq!(result_part1, 198);

        let result_part2 = binary_diagnostic_life_support_rating(input);
        println!("result day_03 part 2: {result_part2}");
        assert_eq!(result_part2, 230);

        Ok(())
    }
}
