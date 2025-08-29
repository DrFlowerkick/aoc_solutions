//!day_15.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    numbers: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.split(',').filter_map(|n| n.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.get_nth_num(2020)
    }
    #[cfg(any(feature = "long-run-time", test))]
    fn solution_part_2(&self) -> u64 {
        // there is probably some clever solution, but we brute force it
        self.get_nth_num(30_000_000)
    }
    fn get_nth_num(&self, nth: u64) -> u64 {
        let mut cache: HashMap<u64, (u64, Option<u64>)> = HashMap::new();
        let mut last_num = 0;
        for turn in 0..nth {
            if (turn as usize) < self.numbers.len() {
                last_num = self.numbers[turn as usize];
                cache.insert(last_num, (turn, None));
            } else {
                let Some(&(last, before_last)) = cache.get(&last_num) else {
                    panic!("last_num must always be held in cache.")
                };
                match before_last {
                    Some(bl) => {
                        // at least second time last_num is seen
                        // insert or update delta in cache
                        last_num = last - bl;
                        cache
                            .entry(last_num)
                            .and_modify(|v| *v = (turn, Some(v.0)))
                            .or_insert((turn, None));
                    }
                    None => {
                        // first time last_num is seen
                        // insert or update zero in cache
                        last_num = 0;
                        cache
                            .entry(last_num)
                            .and_modify(|v| *v = (turn, Some(v.0)))
                            .or_insert((turn, None));
                    }
                }
            }
        }
        last_num
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_15.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 260);

    #[cfg(any(feature = "long-run-time", test))]
    {
        let result_part2 = challenge.solution_part_2();
        println!("result day_15 part 2: {result_part2}");
        assert_eq!(result_part2, 950);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 15 part 2 skipped because of long run time")
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_15() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2020/day_15_example.txt");
        let solutions = [
            (436, 175594),
            (1, 2578),
            (10, 3544142),
            (27, 261214),
            (78, 6895259),
            (438, 18),
            (1836, 362),
        ];
        for (input, (solution_part_1, solution_part_2)) in multi_input.lines().zip(solutions) {
            let example = ChallengeInput::from(input);

            let result_part1 = example.solution_part_1();
            println!("result day_15 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part_1);

            let result_part2 = example.solution_part_2();
            println!("result day_15 part 2: {result_part2}");
            assert_eq!(result_part2, solution_part_2);
        }

        Ok(())
    }
}
