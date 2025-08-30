//!day_19.rs

use anyhow::Result;
use std::collections::HashMap;

enum Rule {
    A,
    B,
    Other(Vec<Vec<u64>>),
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        match value {
            "\"a\"" => Rule::A,
            "\"b\"" => Rule::B,
            _ => Rule::Other(
                value
                    .split(" | ")
                    .map(|numbers| {
                        numbers
                            .split_whitespace()
                            .filter_map(|n| n.parse().ok())
                            .collect()
                    })
                    .collect(),
            ),
        }
    }
}

struct ChallengeInput {
    rules: HashMap<u64, Rule>,
    patterns: Vec<String>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (rules, patterns) = value.split_once("\n\n").unwrap();
        ChallengeInput {
            rules: rules
                .lines()
                .filter_map(|l| l.split_once(": "))
                .map(|(n, r)| (n.parse().unwrap(), Rule::from(r)))
                .collect(),
            patterns: patterns.lines().map(|l| l.to_string()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (usize, usize) {
        let mut cache: HashMap<u64, Vec<String>> = HashMap::new();
        let possible_patterns = self.get_possible_patterns(&0, &mut cache);
        let invalid_patters_no_loop: Vec<String> = self
            .patterns
            .iter()
            .filter(|p| !possible_patterns.contains(p))
            .cloned()
            .collect();
        let result_part_1 = self.patterns.len() - invalid_patters_no_loop.len();
        let Some(p42) = cache.get(&42) else {
            return (result_part_1, 0);
        };
        let Some(p31) = cache.get(&31) else {
            return (result_part_1, 0);
        };
        (
            result_part_1,
            result_part_1
                + invalid_patters_no_loop
                    .iter()
                    .filter(|pat| self.compare_with_p42_p31(pat, p42, p31))
                    .count(),
        )
    }
    fn get_possible_patterns(
        &self,
        rule: &u64,
        cache: &mut HashMap<u64, Vec<String>>,
    ) -> Vec<String> {
        match self.rules.get(rule).unwrap() {
            Rule::A => vec!["a".into()],
            Rule::B => vec!["b".into()],
            Rule::Other(or_rules) => {
                if let Some(patterns) = cache.get(rule) {
                    return patterns.clone();
                }
                let mut patterns: Vec<String> = Vec::new();
                for and_rules in or_rules.iter() {
                    let and_patterns = and_rules.iter().fold(Vec::new(), |pat, next_rule| {
                        let next_patterns = self.get_possible_patterns(next_rule, cache);
                        if pat.is_empty() {
                            next_patterns
                        } else {
                            let mut combined_patterns: Vec<String> = Vec::new();
                            for prev_pat in pat.iter() {
                                for net_pat in next_patterns.iter() {
                                    let combined_pattern = prev_pat.to_owned() + net_pat;
                                    combined_patterns.push(combined_pattern);
                                }
                            }
                            combined_patterns
                        }
                    });
                    patterns.extend(and_patterns);
                }
                cache.insert(*rule, patterns.clone());
                patterns
            }
        }
    }
    fn compare_with_p42_p31(&self, pat: &str, p42: &Vec<String>, p31: &Vec<String>) -> bool {
        // strip p42 and count occurrences of p42. Minimum count is 2, because both rules 8 and 11 contain at least p42 once each
        let (stripped_pat, p42_count) = self.strip_pattern(pat, p42, 0);
        if p42_count < 2 {
            return false;
        }
        // since rule 8 contains at least p42 once and rule 11 contains at least p31 once,
        // occurrences of p31 must be at least one 1 and less than p42_count
        let (remaining_pat, p31_count) = self.strip_pattern(stripped_pat, p31, 0);
        remaining_pat.is_empty() && 1 <= p31_count && p31_count < p42_count
    }
    #[allow(clippy::only_used_in_recursion)]
    fn strip_pattern<'a>(
        &self,
        pat: &'a str,
        patterns: &Vec<String>,
        depth: u64,
    ) -> (&'a str, u64) {
        if let Some(stripped_pat) = patterns.iter().find_map(|prefix| pat.strip_prefix(prefix)) {
            self.strip_pattern(stripped_pat, patterns, depth + 1)
        } else {
            (pat, depth)
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_19.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, 104);

    println!("result day_19 part 2: {result_part2}");
    assert_eq!(result_part2, 314);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_19() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_19_example_1.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, _) = example.solution_part_1_and_2();
        println!("result day_19 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        let input = include_str!("../../../../aoc_input/aoc-2020/day_19_example_2.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_19 part 2: {result_part2}");
        assert_eq!(result_part1, 3);
        assert_eq!(result_part2, 12);

        Ok(())
    }
}
