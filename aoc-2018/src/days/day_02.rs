//!day_02.rs

use anyhow::Result;
use std::collections::HashMap;

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(input: &'a str) -> Self {
        ChallengeInput { input }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u64 {
        let mut count_two = 0;
        let mut count_three = 0;
        for line in self.input.lines() {
            let mut counted: HashMap<char, u64> = HashMap::new();
            for c in line.chars() {
                counted.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
            count_two += if counted.values().any(|v| *v == 2) {
                1
            } else {
                0
            };
            count_three += if counted.values().any(|v| *v == 3) {
                1
            } else {
                0
            };
        }
        count_two * count_three
    }
    fn solution_part_2(&self) -> String {
        for (index, line_a) in self.input.lines().enumerate() {
            'loop_b: for line_b in self.input.lines().skip(index + 1) {
                let mut dif_count = 0;
                let mut dif_index = 0;
                for ((index, a), b) in line_a.char_indices().zip(line_b.chars()) {
                    if a != b {
                        dif_count += 1;
                        if dif_count > 1 {
                            continue 'loop_b;
                        }
                        dif_index = index;
                    }
                }
                if dif_count == 1 {
                    let before = line_a[..dif_index].to_string();
                    return before + &line_a[dif_index + 1..];
                }
            }
        }
        "".into()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, 5_456);

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, "megsdlpulxvinkatfoyzxcbvq");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_02_example_01.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, 12);

        let input = include_str!("../../../../aoc_input/aoc-2018/day_02_example_02.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_02 part 2: {result_part2}");
        assert_eq!(result_part2, "fgij");

        Ok(())
    }
}
