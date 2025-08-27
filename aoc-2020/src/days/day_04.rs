//!day_04.rs

use anyhow::Result;
use std::collections::HashMap;

struct Passport {
    fields: HashMap<String, String>,
}

impl From<&str> for Passport {
    fn from(value: &str) -> Self {
        Passport {
            fields: value
                .split_whitespace()
                .filter_map(|e| e.split_once(':'))
                .map(|(l, r)| (l.to_string(), r.to_string()))
                .collect(),
        }
    }
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_fields
            .into_iter()
            .all(|e| self.fields.keys().any(|k| k == e))
    }
    fn is_valid_part2(&self) -> bool {
        self.is_valid_part1() && {
            self.fields.iter().all(|(k, v)| {
                let v_num_chars = v.chars().count();
                match k.as_str() {
                    "byr" => v_num_chars == 4 && check_digit_value_range(v, 1920, 2002),
                    "iyr" => v_num_chars == 4 && check_digit_value_range(v, 2010, 2020),
                    "eyr" => v_num_chars == 4 && check_digit_value_range(v, 2020, 2030),
                    "hgt" => {
                        if let Some(height_cm) = v.strip_suffix("cm") {
                            check_digit_value_range(height_cm, 150, 193)
                        } else if let Some(height_in) = v.strip_suffix("in") {
                            check_digit_value_range(height_in, 59, 76)
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        v_num_chars == 7
                            && if let Some(color) = v.strip_prefix('#') {
                                color.chars().all(|c| {
                                    c.is_ascii_digit()
                                        || (c.is_lowercase() && c.is_ascii_hexdigit())
                                })
                            } else {
                                false
                            }
                    }
                    "ecl" => {
                        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str())
                    }
                    "pid" => v_num_chars == 9 && v.chars().all(|c| c.is_ascii_digit()),
                    "cid" => true,
                    _ => false,
                }
            })
        }
    }
}

fn check_digit_value_range(digit_str: &str, min: u64, max: u64) -> bool {
    if let Ok(digit) = digit_str.parse::<u64>() {
        digit >= min && digit <= max
    } else {
        false
    }
}

struct ChallengeInput {
    passports: Vec<Passport>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            passports: value.split("\n\n").map(Passport::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.passports.iter().filter(|p| p.is_valid_part1()).count()
    }
    fn solution_part_2(&self) -> usize {
        self.passports.iter().filter(|p| p.is_valid_part2()).count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_04.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 196);

    let result_part2 = challenge.solution_part_2();
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 114);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_04_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        let result_part2 = example.solution_part_2();
        println!("result day_04 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
