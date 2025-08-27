//!day_02.rs

use anyhow::Result;

#[derive(Debug)]
struct Day02Data {
    levels: Vec<Vec<i128>>,
}

impl From<&str> for Day02Data {
    fn from(value: &str) -> Self {
        Self {
            levels: value
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .filter_map(|n| n.parse::<i128>().ok())
                        .collect()
                })
                .collect(),
        }
    }
}

fn is_save_report(report: &[i128], use_dampener: bool, reverse: bool) -> bool {
    let mut last_level: Option<i128> = None;
    let mut direction: Option<bool> = None;
    let mut safe = true;
    let mut dampener = use_dampener;
    let mut level_iter: Box<dyn Iterator<Item = &i128>> = Box::new(report.iter());
    if reverse {
        level_iter = Box::new(report.iter().rev());
    }
    for level in level_iter {
        if let Some(ll) = last_level {
            if (level - ll).abs() > 3 || level - ll == 0 {
                if dampener {
                    dampener = false;
                    // skip current level
                    continue;
                }
                safe = false;
                break;
            }
            match direction {
                Some(dir) => {
                    if dir != (*level > ll) {
                        if dampener {
                            dampener = false;
                            // skip current level
                            continue;
                        }
                        safe = false;
                        break;
                    }
                }
                None => {
                    direction = Some(*level > ll);
                }
            }
        }
        last_level = Some(*level);
    }
    safe
}

impl Day02Data {
    fn count_save_reports(&self, use_dampener: bool) -> usize {
        let mut count = 0;
        for report in self.levels.iter() {
            if is_save_report(report, use_dampener, false) {
                count += 1;
            } else if use_dampener {
                // if report is unsafe even with use_dampener, than perhaps the first element is the bad level.
                // this can be checked by reversing the order of the report levels
                if is_save_report(report, use_dampener, true) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn day_02() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_02.txt");
    let challenge = Day02Data::from(input);

    let result_part1 = challenge.count_save_reports(false);
    println!("result day 02 part 1: {}", result_part1);
    assert_eq!(result_part1, 287);

    let result_part2 = challenge.count_save_reports(true);
    println!("result day 02 part 2: {}", result_part2);
    assert_eq!(result_part2, 354);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_02_example.txt");
        let challenge = Day02Data::from(input);

        let result_part1 = challenge.count_save_reports(false);
        println!("result day 02 part 1: {}", result_part1);
        assert_eq!(result_part1, 2);

        let result_part2 = challenge.count_save_reports(true);
        println!("result day 02 part 2: {}", result_part2);
        assert_eq!(result_part2, 4);

        Ok(())
    }
}
