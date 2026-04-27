//!day_13.rs

use anyhow::Result;
use std::collections::HashMap;

type Firewall = HashMap<u64, (Vec<bool>, bool)>;

struct ChallengeInput {
    firewall: Firewall,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut firewall = HashMap::new();
        for line in value.lines() {
            let (depth, range) = line.split_once(": ").unwrap();
            let depth = depth.parse().unwrap();
            let range = range.parse().unwrap();
            let mut layer = vec![false; range];
            layer[0] = true;
            firewall.insert(depth, (layer, false));
        }
        ChallengeInput { firewall }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut my_depth = 0;
        let max_depth = *self.firewall.keys().max().unwrap();
        let mut severity = 0;
        let mut firewall = self.firewall.clone();
        while my_depth <= max_depth {
            if let Some((layer, _)) = firewall.get(&my_depth)
                && layer[0]
            {
                severity += my_depth * layer.len() as u64;
            }
            for (layer, dir) in firewall.values_mut() {
                let scanner_pos = layer.iter().position(|l| *l).unwrap();
                if *dir {
                    // moving back
                    layer.rotate_left(1);
                    *dir = scanner_pos - 1 > 0;
                } else {
                    // moving forward
                    layer.rotate_right(1);
                    *dir = scanner_pos + 1 == layer.len() - 1;
                }
            }
            my_depth += 1;
        }
        severity
    }
    fn solution_part_2(&self) -> u64 {
        let max_depth = *self.firewall.keys().max().unwrap();
        let mut seen: HashMap<u64, Firewall> = HashMap::new();
        seen.insert(0, self.firewall.clone());
        'outer: for delay in 0..u64::MAX {
            let mut tick = delay;
            let mut my_depth = 0;
            while my_depth <= max_depth {
                // calc next firewall state
                if !seen.contains_key(&(tick + 1)) {
                    let mut firewall = seen.get(&tick).unwrap().clone();

                    for (layer, dir) in firewall.values_mut() {
                        let scanner_pos = layer.iter().position(|l| *l).unwrap();
                        if *dir {
                            // moving back
                            layer.rotate_left(1);
                            *dir = scanner_pos - 1 > 0;
                        } else {
                            // moving forward
                            layer.rotate_right(1);
                            *dir = scanner_pos + 1 == layer.len() - 1;
                        }
                    }

                    seen.insert(tick + 1, firewall);
                }

                if let Some(firewall) = seen.get(&tick)
                    && let Some((layer, _)) = firewall.get(&my_depth)
                    && layer[0]
                {
                    // got caught
                    continue 'outer;
                } else {
                    my_depth += 1;
                }
                tick += 1;
            }
            return delay;
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_13.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 2_164);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, 3_861_798);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_13_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, 24);

        let result_part2 = example.solution_part_2();
        println!("result day_13 part 2: {result_part2}");
        assert_eq!(result_part2, 10);

        Ok(())
    }
}
