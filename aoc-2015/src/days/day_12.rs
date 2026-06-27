//!day_12.rs

use anyhow::Result;
use serde_json::Value;

struct ChallengeInput {
    json: Value,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            json: serde_json::from_str(value).unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        self.parse_and_sum_json_numbers(&self.json, false)
    }
    fn solution_part_2(&self) -> i64 {
        self.parse_and_sum_json_numbers(&self.json, true)
    }
    fn parse_and_sum_json_numbers(&self, json: &Value, check_red: bool) -> i64 {
        if let Some(array) = json.as_array() {
            array
                .iter()
                .map(|j| self.parse_and_sum_json_numbers(j, check_red))
                .sum()
        } else if let Some(object) = json.as_object() {
            if check_red && object.values().any(|v| v == "red") {
                return 0;
            }
            object
                .values()
                .map(|j| self.parse_and_sum_json_numbers(j, check_red))
                .sum()
        } else {
            json.as_i64().unwrap_or_default()
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_12.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 191_164);

    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 87_842);

    Ok(())
}

// no testing because for parsing we use serde_json
