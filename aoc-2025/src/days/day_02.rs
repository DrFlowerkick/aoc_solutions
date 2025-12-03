//!day_02.rs

use anyhow::Result;

struct ChallengeInput {
    ranges: Vec<(u64, u64)>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            ranges: value
                .split(',')
                .filter_map(|r| r.split_once('-'))
                .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut sum_invalid_ids = 0;
        for (start, end) in self.ranges.iter() {
            for id in *start..=*end {
                let str_id = format!("{id}");
                let num_digits = str_id.len();
                if num_digits % 2 == 1 {
                    continue;
                }
                if str_id[..num_digits / 2] == str_id[num_digits / 2..] {
                    // invalid id
                    sum_invalid_ids += id;
                }
            }
        }
        sum_invalid_ids
    }
    fn solution_part_2(&self) -> u64 {
        let mut sum_invalid_ids = 0;
        for (start, end) in self.ranges.iter() {
            'id_loop: for id in *start..=*end {
                let str_id = format!("{id}");
                let num_digits = str_id.len();
                'pattern_loop: for sample_len in 1..=num_digits / 2 {
                    if num_digits % sample_len == 0 {
                        let pattern = &str_id[0..sample_len];
                        for i in 1..num_digits / sample_len {
                            let pos = i * sample_len;
                            if &str_id[pos..pos + sample_len] != pattern {
                                // sub string does not match pattern -> valid id
                                continue 'pattern_loop;
                            }
                        }
                        // invalid id
                        sum_invalid_ids += id;
                        // only add any id once
                        continue 'id_loop;
                    }
                }
            }
        }
        sum_invalid_ids
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, 13_919_717_792);

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, 14_582_313_461);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_02_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, 1_227_775_554);

        let result_part2 = example.solution_part_2();
        println!("result day_02 part 2: {result_part2}");
        assert_eq!(result_part2, 4_174_379_265);

        Ok(())
    }
}
