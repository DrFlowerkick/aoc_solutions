//!day_01.rs

use anyhow::Result;

#[derive(Debug)]
struct Day01Data {
    list_1: Vec<i128>,
    list_2: Vec<i128>,
}

impl From<&str> for Day01Data {
    fn from(value: &str) -> Self {
        let (list_1, list_2): (Vec<i128>, Vec<i128>) = value
            .lines()
            .flat_map(|l| l.split_whitespace())
            .flat_map(|n| n.parse::<i128>())
            .fold((Vec::new(), Vec::new()), |(mut list_1, mut list_2), n| {
                if list_1.len() == list_2.len() {
                    list_1.push(n);
                } else {
                    list_2.push(n);
                }
                (list_1, list_2)
            });
        Self { list_1, list_2 }
    }
}

impl Day01Data {
    fn sort(&mut self) {
        self.list_1.sort();
        self.list_2.sort();
    }

    fn add_up_delta(&self) -> i128 {
        self.list_1
            .iter()
            .zip(self.list_2.iter())
            .map(|(l1, l2)| (l1 - l2).abs())
            .sum()
    }

    fn calc_similarity_score(&self) -> i128 {
        self.list_1
            .iter()
            .map(|l1| {
                l1 * i128::try_from(self.list_2.iter().filter(|l2| l1 == *l2).count()).unwrap()
            })
            .sum()
    }
}

pub fn day_01() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_01.txt");
    let mut challenge = Day01Data::from(input);
    challenge.sort();

    let result_part1 = challenge.add_up_delta();
    println!("result day 01 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_660_292);

    let result_part2 = challenge.calc_similarity_score();
    println!("result day 01 part 2: {}", result_part2);
    assert_eq!(result_part2, 22_776_016);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_01_example.txt");
        let mut challenge = Day01Data::from(input);
        challenge.sort();

        let result_part1 = challenge.add_up_delta();
        println!("result day 01 part 1: {}", result_part1);
        assert_eq!(result_part1, 11);

        let result_part2 = challenge.calc_similarity_score();
        println!("result day 01 part 2: {}", result_part2);
        assert_eq!(result_part2, 31);

        Ok(())
    }
}
