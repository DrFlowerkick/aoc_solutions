//!day_19.rs

use anyhow::{anyhow, Result};
use std::collections::HashMap;

type RuleSet = HashMap<String, Vec<Rule>>;

enum RuleResult {
    Link(String),
    Next,
    Value(u64),
}

#[derive(Default)]
enum RuleMachinePartCartegory {
    #[default]
    ExtremlyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Default)]
enum RuleType {
    #[default]
    Accepted,
    Rejected,
    Greater,
    Less,
    Link,
}

#[derive(Default)]
struct Rule {
    rt: RuleType,
    mpc: RuleMachinePartCartegory,
    comparator_value: u64,
    target_rule: String,
}

impl Rule {
    fn new_accepted() -> Self {
        Self::default()
    }
    fn new_rejected() -> Self {
        Self {
            rt: RuleType::Rejected,
            ..Default::default()
        }
    }
    fn new_link(target_rule: String) -> Self {
        Self {
            rt: RuleType::Link,
            target_rule,
            ..Default::default()
        }
    }
    fn from_str(rule_str: &str) -> Result<Self> {
        if rule_str.contains(['<', '>']) {
            let mpc = match &rule_str[0..1] {
                "x" => RuleMachinePartCartegory::ExtremlyCoolLooking,
                "m" => RuleMachinePartCartegory::Musical,
                "a" => RuleMachinePartCartegory::Aerodynamic,
                "s" => RuleMachinePartCartegory::Shiny,
                _ => return Err(anyhow!("bad input rule machine part cartegory")),
            };
            let rt = match &rule_str[1..2] {
                ">" => RuleType::Greater,
                "<" => RuleType::Less,
                _ => return Err(anyhow!("bad input rule type")),
            };
            let (comparator_value, target_rule) = &rule_str[2..].split_once(':').unwrap();
            let comparator_value = comparator_value.parse::<u64>()?;
            let target_rule = target_rule.to_string();
            Ok(Self {
                rt,
                mpc,
                comparator_value,
                target_rule,
            })
        } else {
            match rule_str {
                "A" => Ok(Rule::new_accepted()),
                "R" => Ok(Rule::new_rejected()),
                _ => Ok(Rule::new_link(rule_str.into())),
            }
        }
    }
    fn execute(&self, machine_part: &MachinePart) -> RuleResult {
        let comparator = match self.mpc {
            RuleMachinePartCartegory::ExtremlyCoolLooking => machine_part.x,
            RuleMachinePartCartegory::Musical => machine_part.m,
            RuleMachinePartCartegory::Aerodynamic => machine_part.a,
            RuleMachinePartCartegory::Shiny => machine_part.s,
        };
        match self.rt {
            RuleType::Accepted => RuleResult::Value(machine_part.sum()),
            RuleType::Rejected => RuleResult::Value(0),
            RuleType::Link => RuleResult::Link(self.target_rule.clone()),
            RuleType::Greater => {
                if comparator > self.comparator_value {
                    RuleResult::Link(self.target_rule.clone())
                } else {
                    RuleResult::Next
                }
            }
            RuleType::Less => {
                if comparator < self.comparator_value {
                    RuleResult::Link(self.target_rule.clone())
                } else {
                    RuleResult::Next
                }
            }
        }
    }
    fn check_range(&self, machine_part_range: &MachinePartRange) -> RuleRangeResult {
        let (min_comparator, max_comparator) = match self.mpc {
            RuleMachinePartCartegory::ExtremlyCoolLooking => {
                (machine_part_range.min.x, machine_part_range.max.x)
            }
            RuleMachinePartCartegory::Musical => {
                (machine_part_range.min.m, machine_part_range.max.m)
            }
            RuleMachinePartCartegory::Aerodynamic => {
                (machine_part_range.min.a, machine_part_range.max.a)
            }
            RuleMachinePartCartegory::Shiny => (machine_part_range.min.s, machine_part_range.max.s),
        };
        match self.rt {
            RuleType::Accepted => RuleRangeResult::Value(machine_part_range.combinations()),
            RuleType::Rejected => RuleRangeResult::Value(0),
            RuleType::Link => RuleRangeResult::Link(self.target_rule.clone()),
            RuleType::Greater => {
                if min_comparator > self.comparator_value {
                    RuleRangeResult::Link(self.target_rule.clone())
                } else if max_comparator <= self.comparator_value {
                    RuleRangeResult::Next
                } else {
                    // split machine_part_range
                    let (accepted_machine_part_range, rejected_machine_part_range) =
                        machine_part_range
                            .split_at_greater_comparator_value(&self.mpc, self.comparator_value);
                    RuleRangeResult::Split(
                        accepted_machine_part_range,
                        self.target_rule.clone(),
                        rejected_machine_part_range,
                    )
                }
            }
            RuleType::Less => {
                if max_comparator < self.comparator_value {
                    RuleRangeResult::Link(self.target_rule.clone())
                } else if min_comparator >= self.comparator_value {
                    RuleRangeResult::Next
                } else {
                    // split machine_part_range
                    let (accepted_machine_part_range, rejected_machine_part_range) =
                        machine_part_range
                            .split_at_less_comparator_value(&self.mpc, self.comparator_value);
                    RuleRangeResult::Split(
                        accepted_machine_part_range,
                        self.target_rule.clone(),
                        rejected_machine_part_range,
                    )
                }
            }
        }
    }
}

fn rule_set_from_str(rules: &str) -> Result<RuleSet> {
    let mut rule_set = RuleSet::new();
    rule_set.insert(String::from("A"), vec![Rule::new_accepted()]);
    rule_set.insert(String::from("R"), vec![Rule::new_rejected()]);
    for rule_line in rules.lines() {
        let (rule_label, rule_list) = rule_line
            .split_once('{')
            .map(|(label, list)| {
                (
                    label.to_string(),
                    list.strip_suffix('}')
                        .unwrap()
                        .split(',')
                        .collect::<Vec<&str>>(),
                )
            })
            .unwrap();
        let mut rule_collection: Vec<Rule> = Vec::new();
        for rule in rule_list.iter() {
            rule_collection.push(Rule::from_str(rule)?);
        }
        rule_set.insert(rule_label, rule_collection);
    }
    Ok(rule_set)
}

#[derive(Debug, Clone, Copy)]
struct MachinePart {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl From<&str> for MachinePart {
    fn from(value: &str) -> Self {
        let values: Vec<u64> = value
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|s| {
                s.split_once('=')
                    .unwrap()
                    .1
                    .parse::<u64>()
                    .expect("bad machine part input")
            })
            .collect();
        Self {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }
}

impl MachinePart {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
    fn calc_machine_part_rating(&self, rule_set: &RuleSet) -> Result<u64> {
        self.calc_machine_part_rating_recursive(String::from("in"), rule_set)
    }
    fn calc_machine_part_rating_recursive(
        &self,
        rule_key: String,
        rule_set: &RuleSet,
    ) -> Result<u64> {
        match rule_set.get(&rule_key) {
            Some(rules) => {
                for rule in rules.iter() {
                    match rule.execute(self) {
                        RuleResult::Link(new_rule_key) => {
                            return self.calc_machine_part_rating_recursive(new_rule_key, rule_set)
                        }
                        RuleResult::Next => (),
                        RuleResult::Value(value) => return Ok(value),
                    }
                }
                Err(anyhow!("rule did not result in RuleResult::Value()"))
            }
            None => Err(anyhow!("rule key not found")),
        }
    }
}

enum RuleRangeResult {
    // rule splits range, 1. half is accepted and moves to linked rule, 2. half is rejected and moves to next rule
    Split(MachinePartRange, String, MachinePartRange),
    // rule applies to full remaining range
    Link(String),
    // rule rejects full remaining range
    Next,
    // range is either accepted or rejected and possible combinations is returned
    Value(u64),
}

#[derive(Clone, Copy)]
struct MachinePartRange {
    min: MachinePart,
    max: MachinePart,
}

impl MachinePartRange {
    fn new() -> Self {
        Self {
            min: MachinePart {
                x: 1,
                m: 1,
                a: 1,
                s: 1,
            },
            max: MachinePart {
                x: 4000,
                m: 4000,
                a: 4000,
                s: 4000,
            },
        }
    }
    fn combinations(&self) -> u64 {
        (self.max.x - self.min.x + 1)
            * (self.max.m - self.min.m + 1)
            * (self.max.a - self.min.a + 1)
            * (self.max.s - self.min.s + 1)
    }
    fn calc_machine_part_combinations(&mut self, rule_set: &RuleSet) -> Result<u64> {
        self.calc_machine_part_combinations_recursive(String::from("in"), rule_set)
    }
    fn split_at_greater_comparator_value(
        &self,
        mpc: &RuleMachinePartCartegory,
        comparator_value: u64,
    ) -> (Self, Self) {
        let mut accepted_range = *self;
        let mut rejected_range = *self;
        match mpc {
            RuleMachinePartCartegory::ExtremlyCoolLooking => {
                accepted_range.min.x = comparator_value + 1;
                rejected_range.max.x = comparator_value;
            }
            RuleMachinePartCartegory::Musical => {
                accepted_range.min.m = comparator_value + 1;
                rejected_range.max.m = comparator_value;
            }
            RuleMachinePartCartegory::Aerodynamic => {
                accepted_range.min.a = comparator_value + 1;
                rejected_range.max.a = comparator_value;
            }
            RuleMachinePartCartegory::Shiny => {
                accepted_range.min.s = comparator_value + 1;
                rejected_range.max.s = comparator_value;
            }
        }
        (accepted_range, rejected_range)
    }
    fn split_at_less_comparator_value(
        &self,
        mpc: &RuleMachinePartCartegory,
        comparator_value: u64,
    ) -> (Self, Self) {
        let mut accepted_range = *self;
        let mut rejected_range = *self;
        match mpc {
            RuleMachinePartCartegory::ExtremlyCoolLooking => {
                accepted_range.max.x = comparator_value - 1;
                rejected_range.min.x = comparator_value;
            }
            RuleMachinePartCartegory::Musical => {
                accepted_range.max.m = comparator_value - 1;
                rejected_range.min.m = comparator_value;
            }
            RuleMachinePartCartegory::Aerodynamic => {
                accepted_range.max.a = comparator_value - 1;
                rejected_range.min.a = comparator_value;
            }
            RuleMachinePartCartegory::Shiny => {
                accepted_range.max.s = comparator_value - 1;
                rejected_range.min.s = comparator_value;
            }
        }
        (accepted_range, rejected_range)
    }
    fn calc_machine_part_combinations_recursive(
        mut self,
        rule_key: String,
        rule_set: &RuleSet,
    ) -> Result<u64> {
        let mut combinations = 0;
        match rule_set.get(&rule_key) {
            Some(rules) => {
                for rule in rules.iter() {
                    match rule.check_range(&self) {
                        RuleRangeResult::Split(accecpted_range, new_rule_key, rejected_range) => {
                            combinations += accecpted_range
                                .calc_machine_part_combinations_recursive(new_rule_key, rule_set)?;
                            self = rejected_range;
                        }
                        RuleRangeResult::Link(new_rule_key) => {
                            combinations += self
                                .calc_machine_part_combinations_recursive(new_rule_key, rule_set)?
                        }
                        RuleRangeResult::Next => (),
                        RuleRangeResult::Value(value) => return Ok(combinations + value),
                    }
                }
                Ok(combinations)
            }
            None => Err(anyhow!("rule key not found")),
        }
    }
}

pub fn day_19() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_19.txt");
    let (rules, machine_parts) = input.split_once("\n\n").unwrap();
    let rule_set = rule_set_from_str(rules)?;
    let machine_parts: Vec<MachinePart> = machine_parts.lines().map(MachinePart::from).collect();
    let mut result_part1 = 0;
    for machine_part in machine_parts.iter() {
        result_part1 += machine_part.calc_machine_part_rating(&rule_set)?;
    }
    println!("result day 19 part 1: {}", result_part1);
    assert_eq!(result_part1, 383_682);

    let mut machine_part_range = MachinePartRange::new();
    let result_part2 = machine_part_range.calc_machine_part_combinations(&rule_set)?;
    eprintln!("result day 19 part 2: {}", result_part2);
    assert_eq!(result_part2, 117_954_800_808_317);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part1() -> Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}\n\
                           pv{a>1716:R,A}\n\
                           lnx{m>1548:A,A}\n\
                           rfg{s<537:gd,x>2440:R,A}\n\
                           qs{s>3448:A,lnx}\n\
                           qkq{x<1416:A,crn}\n\
                           crn{x>2662:A,R}\n\
                           in{s<1351:px,qqz}\n\
                           qqz{s>2770:qs,m<1801:hdj,R}\n\
                           gd{a>3333:R,R}\n\
                           hdj{m>838:A,pv}\n\n\
                           {x=787,m=2655,a=1222,s=2876}\n\
                           {x=1679,m=44,a=2067,s=496}\n\
                           {x=2036,m=264,a=79,s=2244}\n\
                           {x=2461,m=1339,a=466,s=291}\n\
                           {x=2127,m=1623,a=2188,s=1013}";
        let (rules, machine_parts) = input.split_once("\n\n").unwrap();
        let rule_set = rule_set_from_str(rules)?;
        let machine_parts: Vec<MachinePart> =
            machine_parts.lines().map(MachinePart::from).collect();
        let mut result_part1 = 0;
        for machine_part in machine_parts.iter() {
            result_part1 += machine_part.calc_machine_part_rating(&rule_set)?;
        }
        eprintln!("result day 19 example part 1: {}", result_part1);
        assert_eq!(result_part1, 19_114);

        let mut machine_part_range = MachinePartRange::new();
        let result_part2 = machine_part_range.calc_machine_part_combinations(&rule_set)?;
        eprintln!("result day 19 example part 2: {}", result_part2);
        assert_eq!(result_part2, 167_409_079_868_000);

        Ok(())
    }
}
