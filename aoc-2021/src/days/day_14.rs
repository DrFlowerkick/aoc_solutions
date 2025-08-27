//!day_14.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    pairs: HashMap<String, u64>,
    pair_insertion_rules: HashMap<String, String>,
    last_char: char,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (pairs_str, pair_insertion_rules) = value.split_once("\n\n").unwrap();
        let mut pairs: HashMap<String, u64> = HashMap::new();
        pairs_str
            .chars()
            .zip(pairs_str.chars().skip(1))
            .map(|(l, r)| String::from_iter([l, r]))
            .for_each(|pair| {
                pairs.entry(pair).and_modify(|v| *v += 1).or_insert(1);
            });
        let pair_insertion_rules = pair_insertion_rules
            .lines()
            .filter_map(|line| line.split_once(" -> "))
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect();
        ChallengeInput {
            pairs,
            pair_insertion_rules,
            last_char: pairs_str.chars().last().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> u64 {
        self.analyze_n_steps(10)
    }
    fn solution_part_2(&mut self) -> u64 {
        // do 30 more steps
        self.analyze_n_steps(30)
    }
    fn one_step(&mut self) {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            let insert = self.pair_insertion_rules.get(pair).unwrap().to_owned();
            let left_insert = pair[0..1].to_string() + insert.as_str();
            new_pairs
                .entry(left_insert)
                .and_modify(|n| *n += count)
                .or_insert(*count);
            let insert_right = insert + &pair[1..];
            new_pairs
                .entry(insert_right)
                .and_modify(|n| *n += count)
                .or_insert(*count);
        }
        self.pairs = new_pairs;
    }
    fn analyze_n_steps(&mut self, steps: usize) -> u64 {
        for _ in 1..=steps {
            self.one_step();
        }
        let mut element_count: HashMap<char, u64> = HashMap::new();
        element_count.insert(self.last_char, 1);
        for (pair, count) in self.pairs.iter() {
            for element in pair.chars().take(1) {
                element_count
                    .entry(element)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        let max_element = *element_count.values().max().unwrap();
        let min_element = *element_count.values().min().unwrap();
        max_element - min_element
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_14.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, 2_233);

    let result_part2 = challenge.solution_part_2();
    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 2_884_513_602_164);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one_step() {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_14_example.txt");
        let mut example = ChallengeInput::from(input);
        println!("{:?}", example.pairs);
        example.one_step();
        println!("{:?}", example.pairs);
        assert_eq!(example.pairs.len(), 6);
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_14_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_14 part 1: {result_part1}");
        assert_eq!(result_part1, 1588);

        let result_part2 = example.solution_part_2();
        println!("result day_14 part 2: {result_part2}");
        assert_eq!(result_part2, 2_188_189_693_529);

        Ok(())
    }
}
