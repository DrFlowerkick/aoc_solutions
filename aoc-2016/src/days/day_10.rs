//!day_10.rs

use anyhow::Result;
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy)]
enum Action {
    Bot(u64),
    Output(u64),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        let (action, target) = value.split_once(' ').unwrap();
        if action == "bot" {
            Action::Bot(target.parse().unwrap())
        } else {
            Action::Output(target.parse().unwrap())
        }
    }
}

#[derive(Clone)]
struct Bot {
    id: u64,
    values: BTreeSet<u64>,
    low: Action,
    high: Action,
}

impl From<&str> for Bot {
    fn from(value: &str) -> Self {
        let (id, rem) = value.split_once(" gives low to ").unwrap();
        let (low, high) = rem.split_once(" and high to ").unwrap();
        Bot {
            id: id.parse().unwrap(),
            values: BTreeSet::new(),
            low: low.into(),
            high: high.into(),
        }
    }
}

impl Bot {
    fn add_value(&mut self, value: u64) {
        self.values.insert(value);
        assert!(self.values.len() < 3);
    }
    fn ready_for_action(&self) -> bool {
        self.values.len() == 2
    }
    fn pop_low(&mut self) -> Option<(u64, Action)> {
        if let Some(low) = self.values.pop_first() {
            Some((low, self.low))
        } else {
            None
        }
    }
    fn pop_high(&mut self) -> Option<(u64, Action)> {
        if let Some(high) = self.values.pop_last() {
            Some((high, self.high))
        } else {
            None
        }
    }
}

struct ChallengeInput {
    bots: HashMap<u64, Bot>,
    outputs: HashMap<u64, u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut values: Vec<&str> = Vec::new();
        let mut bots = HashMap::new();
        for line in value.lines() {
            if let Some(bot_line) = line.strip_prefix("bot ") {
                let bot = Bot::from(bot_line);
                bots.insert(bot.id, bot);
            } else {
                values.push(line);
            }
        }
        for value_line in values {
            let mut value_iter = value_line
                .split_whitespace()
                .filter_map(|d| d.parse::<u64>().ok());
            let v = value_iter.next().unwrap();
            let bot_id = value_iter.next().unwrap();
            if let Some(bot) = bots.get_mut(&bot_id) {
                bot.add_value(v);
            }
        }

        ChallengeInput {
            bots,
            outputs: HashMap::new(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self, low: u64, high: u64) -> u64 {
        let mut value_handler = None::<u64>;
        loop {
            let active_bot_ids: Vec<u64> = self
                .bots
                .iter()
                .filter(|(_, b)| b.ready_for_action())
                .map(|(i, _)| *i)
                .collect();
            if active_bot_ids.is_empty() {
                break;
            }
            for bot_id in active_bot_ids {
                let bot = self.bots.get_mut(&bot_id).unwrap();
                let (bot_low, low_action) = bot.pop_low().unwrap();
                let (bot_high, high_action) = bot.pop_high().unwrap();
                if bot_low == low && bot_high == high && value_handler.is_none() {
                    value_handler = Some(bot_id);
                }
                self.apply_action(low_action, bot_low);
                self.apply_action(high_action, bot_high);
            }
        }
        value_handler.unwrap()
    }
    fn apply_action(&mut self, action: Action, value: u64) {
        match action {
            Action::Bot(bot_id) => {
                self.bots.get_mut(&bot_id).unwrap().add_value(value);
            }
            Action::Output(output_id) => {
                self.outputs
                    .entry(output_id)
                    .and_modify(|v| *v = value)
                    .or_insert(value);
            }
        }
    }
    fn solution_part_2(&self) -> u64 {
        self.outputs
            .iter()
            .filter(|(k, _)| (0_u64..=2).contains(k))
            .map(|(_, v)| *v)
            .product()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_10.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(17, 61);
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 27);

    let result_part2 = challenge.solution_part_2();
    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 13_727);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_10_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(2, 5);
        println!("result day_10 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        let result_part2 = example.solution_part_2();
        println!("result day_10 part 2: {result_part2}");
        assert_eq!(result_part2, 30);

        Ok(())
    }
}
