//!day_05.rs

use anyhow::Result;
use rayon::prelude::*;

struct ChallengeInput {
    input: String,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            input: value.to_string(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (String, String) {
        let mut part_1 = String::new();
        let mut part_2 = [' '; 8];
        let mut part_2_count = 0;
        let chunk_size = 500_000;
        let mut start = 0u64;
        while part_1.len() < 8 || part_2_count < 8 {
            let end = start + chunk_size;
            let hits: Vec<String> = (start..end)
                .into_par_iter()
                .filter_map(|count| {
                    let data = format!("{}{}", self.input, count);
                    let digest = md5::compute(data);
                    if digest[0] == 0 && digest[1] == 0 && digest[2] <= 0x0f {
                        Some(format!("{:x}", digest))
                    } else {
                        None
                    }
                })
                .collect();

            for hit in hits {
                let c6 = hit.chars().nth(5).unwrap();
                if part_1.len() < 8 {
                    part_1.push(c6);
                }
                if let Some(digit) = c6.to_digit(10)
                    && digit < 8
                    && part_2[digit as usize] == ' '
                {
                    part_2_count += 1;
                    part_2[digit as usize] = hit.chars().nth(6).unwrap();
                }
            }

            start += chunk_size;
        }
        (part_1, part_2.iter().copied().collect())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_05.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, "f97c354d");

    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, "863dde27");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_05() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_05_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, "18f47a30");

        println!("result day_05 part 2: {result_part2}");
        assert_eq!(result_part2, "05ace8e3");

        Ok(())
    }
}
