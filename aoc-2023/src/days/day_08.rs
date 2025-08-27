//!day_08.RS

use anyhow::Result;
use num::integer::lcm;
use std::collections::HashMap;

struct Instructions {
    left: String,
    right: String,
}

impl Instructions {
    fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
    fn direction(&self, direction: bool) -> &String {
        if direction {
            &self.right
        } else {
            &self.left
        }
    }
}

pub fn day_08() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_08.txt");
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let directions = lines.next().unwrap();
    let mut map: HashMap<String, Instructions> = HashMap::new();
    for line in lines {
        let (key, value) = line.split_once('=').unwrap();
        let trim_value = |c: char| !c.is_alphabetic();
        let (left, right) = value.trim_matches(trim_value).split_once(',').unwrap();
        map.insert(
            key.trim().to_string(),
            Instructions::new(left.trim().to_string(), right.trim().to_string()),
        );
    }

    // part 1
    let mut current_key = String::from("AAA");
    let mut steps: u64 = 0;
    // direction: false -> left; true -> right
    for direction in directions.chars().map(|c| c == 'R').cycle() {
        steps += 1;
        current_key = map
            .get(&current_key)
            .unwrap()
            .direction(direction)
            .to_owned();
        if current_key == "ZZZ" {
            break;
        }
    }
    println!("result day 08 part 1: {}", steps);
    assert_eq!(steps, 13_301);

    // part 2
    // hint for solution: every path from each starting node to it's corresponding ending node
    // cycles in a fixed cycle, respectivly.
    // the solution is to identify the cycles and than calc the Lowest Common Multiple (LCM) of them
    let current_keys: Vec<String> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_owned())
        .collect();
    let mut steps_per_key: Vec<u64> = Vec::with_capacity(current_keys.len());

    for start_key in current_keys.iter() {
        current_key = start_key.clone();
        steps = 0;
        // direction: false -> left; true -> right
        for direction in directions.chars().map(|c| c == 'R').cycle() {
            steps += 1;
            current_key = map
                .get(&current_key)
                .unwrap()
                .direction(direction)
                .to_owned();

            if current_key.ends_with('Z') {
                break;
            }
        }
        steps_per_key.push(steps);
    }

    let mut step_iter = steps_per_key.iter();
    let mut lcm_step = *step_iter.next().unwrap();
    for next_key in step_iter {
        lcm_step = lcm(lcm_step, *next_key);
    }
    println!("result day 08 part 2: {}", lcm_step);
    assert_eq!(lcm_step, 7_309_459_565_207);

    Ok(())
}
