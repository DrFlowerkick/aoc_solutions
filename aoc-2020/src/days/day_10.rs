//!day_10.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput {
    adapters: Vec<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            adapters: value.lines().filter_map(|n| n.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> u64 {
        let mut count_dif_1 = 0;
        let mut count_dif_3 = 0;
        self.adapters.push(0);
        self.adapters.sort();
        let max = *self.adapters.last().unwrap();
        self.adapters.push(max + 3);
        for (i, n) in self.adapters[..self.adapters.len() - 1].iter().enumerate() {
            match self.adapters[i + 1] - *n {
                3 => count_dif_3 += 1,
                2 => (),
                1 => count_dif_1 += 1,
                _ => panic!("unexpected difference"),
            }
        }
        count_dif_1 * count_dif_3
    }
    fn solution_part_2(&self) -> u64 {
        let mut seen: HashMap<usize, u64> = HashMap::new();
        self.count_adapters(0, &mut seen)
    }
    fn count_adapters(&self, adapter_index: usize, seen: &mut HashMap<usize, u64>) -> u64 {
        if adapter_index == self.adapters.len() - 1 {
            return 1;
        }
        if let Some(count) = seen.get(&adapter_index) {
            return *count;
        }
        let adapter_jolts = self.adapters[adapter_index];
        let mut count = 0;
        for child_index in (adapter_index + 1..=adapter_index + 3)
            .filter(|i| self.adapters.get(*i).is_some())
            .filter(|i| self.adapters[*i] <= adapter_jolts + 3)
        {
            count += self.count_adapters(child_index, seen);
        }
        seen.insert(adapter_index, count);
        count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_10.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 1_836);

    let result_part2 = challenge.solution_part_2();
    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 43_406_276_662_336);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_10_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_10 part 1: {result_part1}");
        assert_eq!(result_part1, 220);

        let result_part2 = example.solution_part_2();
        println!("result day_10 part 2: {result_part2}");
        assert_eq!(result_part2, 19_208);

        Ok(())
    }
}
