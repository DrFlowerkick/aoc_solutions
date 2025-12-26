//!day_04.rs

use anyhow::Result;
use chrono::{DateTime, Timelike, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Guard {
    ShiftStart(u64),
    FallsAsleep,
    WakesUp,
}

impl From<&str> for Guard {
    fn from(value: &str) -> Self {
        if value.starts_with("Guard") {
            let (_, id) = value.split_once('#').unwrap();
            let id = id.strip_suffix(" begins shift").unwrap().parse().unwrap();
            Self::ShiftStart(id)
        } else if value.starts_with("falls") {
            Self::FallsAsleep
        } else {
            Self::WakesUp
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LogEntry {
    timestamp: DateTime<Utc>,
    guard: Guard,
}

impl From<&str> for LogEntry {
    fn from(value: &str) -> Self {
        let (time_stamp, guard) = value.split_once("] ").unwrap();
        LogEntry {
            timestamp: DateTime::parse_from_str(
                &format!("{} +0000", &time_stamp[1..]),
                "%Y-%m-%d %H:%M %z",
            )
            .unwrap()
            .to_utc(),
            guard: guard.into(),
        }
    }
}

#[derive(Debug)]
struct ChallengeInput {
    log: Vec<LogEntry>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut ci = ChallengeInput {
            log: value.lines().map(LogEntry::from).collect(),
        };
        ci.log.sort_by_key(|a| a.timestamp);
        ci
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let mut sleeping_guards: HashMap<u64, Vec<(u32, u32)>> = HashMap::new();
        let mut current_guard = None::<u64>;
        let mut falls_asleep = None::<u32>;
        for log_entry in self.log.iter() {
            match log_entry.guard {
                Guard::ShiftStart(id) => {
                    current_guard = Some(id);
                    falls_asleep = None;
                }
                Guard::FallsAsleep => {
                    falls_asleep = Some(log_entry.timestamp.minute());
                }
                Guard::WakesUp => {
                    if let Some(guard_id) = current_guard
                        && let Some(fa) = falls_asleep
                    {
                        let wa = log_entry.timestamp.minute();
                        sleeping_guards
                            .entry(guard_id)
                            .and_modify(|sleeping_ranges| sleeping_ranges.push((fa, wa)))
                            .or_insert(vec![(fa, wa)]);
                    }
                }
            }
        }
        let mut sleeping_minutes: HashMap<(u64, u32), u64> = HashMap::new();
        for (guard, sleeping_ranges) in sleeping_guards.iter() {
            for &(fa, wa) in sleeping_ranges.iter() {
                for minute in fa..wa {
                    sleeping_minutes
                        .entry((*guard, minute))
                        .and_modify(|m| *m += 1)
                        .or_insert(1);
                }
            }
        }
        // part 1
        let (most_sleepy_guard, _) = sleeping_guards
            .iter()
            .max_by_key(|(_, sr)| sr.iter().map(|(fa, wa)| *wa - *fa).sum::<u32>())
            .unwrap();
        let ((_, most_sleepy_minute), _) = sleeping_minutes
            .iter()
            .filter(|((g, _), _)| g == most_sleepy_guard)
            .max_by_key(|(_, c)| **c)
            .unwrap();
        let part_1 = *most_sleepy_guard * *most_sleepy_minute as u64;
        // part 2
        let ((most_sleepy_guard, most_sleepy_minute), _) =
            sleeping_minutes.iter().max_by_key(|(_, c)| **c).unwrap();
        (part_1, *most_sleepy_guard * *most_sleepy_minute as u64)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_04.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 26_281);

    println!("result day_04 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_04_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, 240);

        println!("result day_04 part 2: {result_part2}");
        assert_eq!(result_part2, 4455);

        Ok(())
    }
}
