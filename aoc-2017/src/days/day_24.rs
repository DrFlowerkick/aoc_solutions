//!day_24.rs

use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Component {
    port_a: u64,
    port_b: u64,
}

impl From<&str> for Component {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once("/").unwrap();
        Self {
            port_a: a.parse().unwrap(),
            port_b: b.parse().unwrap(),
        }
    }
}

impl Component {
    fn has_partner_of(&self, port: u64) -> Option<u64> {
        if self.port_a == port {
            Some(self.port_b)
        } else if self.port_b == port {
            Some(self.port_a)
        } else {
            None
        }
    }
    fn value(&self) -> u64 {
        self.port_a + self.port_b
    }
}

struct ChallengeInput {
    components: HashSet<Component>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            components: value.lines().map(Component::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let mut chain = Vec::new();
        let mut strength_map: HashMap<usize, u64> = HashMap::new();
        self.link_next_component(0, &mut chain, &mut strength_map);
        let max_strength = strength_map.values().max().unwrap();
        let max_length = strength_map.keys().max().unwrap();
        (
            *max_strength,
            strength_map
                .iter()
                .filter(|(k, _)| *k == max_length)
                .map(|(_, v)| *v)
                .max()
                .unwrap(),
        )
    }
    fn link_next_component(
        &self,
        port: u64,
        chain: &mut Vec<Component>,
        strength_map: &mut HashMap<usize, u64>,
    ) {
        let chain_set: HashSet<Component> = chain.iter().copied().collect();
        let mut last = true;
        for c in self.components.difference(&chain_set) {
            if let Some(p) = c.has_partner_of(port) {
                chain.push(*c);
                self.link_next_component(p, chain, strength_map);
                chain.pop();
                last = false;
            }
        }
        if last {
            let strength: u64 = chain.iter().map(|c| c.value()).sum();
            strength_map
                .entry(chain.len())
                .and_modify(|v| *v = strength.max(*v))
                .or_insert(strength);
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_24.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_24 part 1: {result_part1}");
    assert_eq!(result_part1, 1_511);

    println!("result day_24 part 2: {result_part2}");
    assert_eq!(result_part2, 1_471);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_24() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_24_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_24 part 1: {result_part1}");
        assert_eq!(result_part1, 31);

        println!("result day_24 part 2: {result_part2}");
        assert_eq!(result_part2, 19);

        Ok(())
    }

    #[test]
    fn test_unique_components() {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_24.txt");
        let challenge = ChallengeInput::from(input);

        for (i, a) in challenge.components.iter().enumerate() {
            for b in challenge.components.iter().skip(i + 1) {
                if let Some(p) = b.has_partner_of(a.port_a) {
                    assert_ne!(p, a.port_b);
                }
            }
        }
    }
}
