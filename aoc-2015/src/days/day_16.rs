//!day_16.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Sue<'a> {
    compounds: HashMap<&'a str, u8>,
}

impl<'a> From<&'a str> for Sue<'a> {
    fn from(value: &'a str) -> Self {
        Sue {
            compounds: value
                .split(", ")
                .filter_map(|s| s.split_once(": "))
                .map(|(n, v)| (n, v.parse().unwrap()))
                .collect(),
        }
    }
}

impl<'a> Sue<'a> {
    fn may_be_sue(&self, mfcsam: &Self) -> bool {
        for (key, value) in self.compounds.iter() {
            if let Some(v) = mfcsam.compounds.get(key)
                && v != value
            {
                return false;
            }
        }
        true
    }
    fn may_be_sue_part_2(&self, mfcsam: &Self) -> bool {
        for (key, value) in self.compounds.iter() {
            if let Some(v) = mfcsam.compounds.get(key)
                && match *key {
                    "cats" | "trees" => value <= v,
                    "pomeranians" | "goldfish" => value >= v,
                    _ => v != value,
                }
            {
                return false;
            }
        }
        true
    }
}

struct ChallengeInput<'a> {
    mfcsam: Sue<'a>,
    all_sues: HashMap<u16, Sue<'a>>,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let (mfcsam, all_sues) = value.split_once("\n\n").unwrap();
        ChallengeInput {
            mfcsam: Sue::from(mfcsam),
            all_sues: all_sues
                .lines()
                .filter_map(|l| l.split_once(": "))
                .map(|(n, s)| {
                    let number = n.strip_prefix("Sue ").unwrap();
                    (number.parse().unwrap(), Sue::from(s))
                })
                .collect(),
        }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u16 {
        *self
            .all_sues
            .iter()
            .find(|(_, s)| s.may_be_sue(&self.mfcsam))
            .unwrap()
            .0
    }
    fn solution_part_2(&self) -> u16 {
        *self
            .all_sues
            .iter()
            .find(|(_, s)| s.may_be_sue_part_2(&self.mfcsam))
            .unwrap()
            .0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_16.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, 373);

    let result_part2 = challenge.solution_part_2();
    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, 260);

    Ok(())
}
