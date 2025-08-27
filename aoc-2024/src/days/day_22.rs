//!day_22.rs

use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Day22Data {
    secrets: Vec<u128>,
}

impl From<&str> for Day22Data {
    fn from(value: &str) -> Self {
        Self {
            secrets: value.lines().map(|v| v.parse::<u128>().unwrap()).collect(),
        }
    }
}

impl Day22Data {
    fn sum_up_new_secrets(&self, generations: usize) -> (u128, u128) {
        let mut cache: HashMap<(u128, usize), u128> = HashMap::new();
        let mut seq_cache: HashMap<VecDeque<i8>, u128> = HashMap::new();
        let mut sum_new_secrets = 0;
        for secret in self.secrets.iter() {
            let mut seen: HashSet<VecDeque<i8>> = HashSet::new();
            sum_new_secrets += calc_secret(
                *secret,
                generations,
                &mut cache,
                VecDeque::new(),
                &mut seq_cache,
                &mut seen,
            );
        }
        let bananas = seq_cache.values().max().unwrap();
        (sum_new_secrets, *bananas)
    }
}

fn calc_secret(
    secret: u128,
    remaining_generations: usize,
    cache: &mut HashMap<(u128, usize), u128>,
    mut sequence: VecDeque<i8>,
    seq_cache: &mut HashMap<VecDeque<i8>, u128>,
    seen: &mut HashSet<VecDeque<i8>>,
) -> u128 {
    if remaining_generations == 0 {
        return secret;
    }
    if let Some(cached_secret) = cache.get(&(secret, remaining_generations)) {
        return *cached_secret;
    }
    // multiplying the secret number by 64 -> shift 6 bits to the left
    let shift_secret = secret << 6;
    // mix + prune (prune module 16777216 is equal to & (16777216 - 1), which is & (2^24 - 1), which is 24 true bits
    let mut new_secret = (shift_secret ^ secret) & (16_777_216 - 1);
    // dividing the secret number by 32 -> shift 5 bits to the right
    let shift_secret = new_secret >> 5;
    // mix + prune (prune module 16777216 is equal to & (16777216 - 1), which is & (2^24 - 1), which is 24 true bits
    new_secret = (shift_secret ^ new_secret) & (16_777_216 - 1);
    // multiplying the secret number by 2048 -> shift 11 bits to the left
    let shift_secret = new_secret << 11;
    // mix + prune (prune module 16777216 is equal to & (16777216 - 1), which is & (2^24 - 1), which is 24 true bits
    new_secret = (shift_secret ^ new_secret) & (16_777_216 - 1);
    cache.insert((secret, remaining_generations), new_secret);

    // Part 2
    let secret_digit_0 = (secret % 10) as i8;
    let new_secret_digit_0 = new_secret % 10;
    let delta = (new_secret_digit_0 as i8) - secret_digit_0;
    let full_sequence = match sequence.len() {
        0..=2 => {
            sequence.push_back(delta);
            false
        }
        3 => {
            sequence.push_back(delta);
            true
        }
        4 => {
            sequence.pop_front();
            sequence.push_back(delta);
            true
        }
        _ => unreachable!("size can never be greater 4."),
    };
    if full_sequence && seen.insert(sequence.clone()) {
        seq_cache
            .entry(sequence.clone())
            .and_modify(|v| *v += new_secret_digit_0)
            .or_insert(new_secret_digit_0);
    }
    calc_secret(
        new_secret,
        remaining_generations - 1,
        cache,
        sequence,
        seq_cache,
        seen,
    )
}

pub fn day_22() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_22.txt");
    let challenge = Day22Data::from(input);

    let (result_part1, result_part2) = challenge.sum_up_new_secrets(2000);
    println!("result day 22 part 1: {}", result_part1);
    assert_eq!(result_part1, 13_753_970_725);

    println!("result day 22 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_570);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_secret() {
        let mut cache: HashMap<(u128, usize), u128> = HashMap::new();
        let mut seq_cache: HashMap<VecDeque<i8>, u128> = HashMap::new();
        let mut seen: HashSet<VecDeque<i8>> = HashSet::new();
        let secret = 123_u128;
        assert_eq!(secret % 10, 3);
        let secret = calc_secret(
            secret,
            9,
            &mut cache,
            VecDeque::new(),
            &mut seq_cache,
            &mut seen,
        );
        assert_eq!(secret, 7753432);
        assert_eq!(secret % 10, 2);
        let bananas = seq_cache.values().max().unwrap();
        assert_eq!(bananas, &6);
        let (sequence, _) = seq_cache.iter().find(|(_, b)| *b == bananas).unwrap();
        assert_eq!(sequence, &[-1, -1, 0, 2]);
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_22_example.txt");
        let challenge = Day22Data::from(input);

        let (result_part1, _) = challenge.sum_up_new_secrets(2000);
        println!("result day 22 part 1: {}", result_part1);
        assert_eq!(result_part1, 37_327_623);

        let challenge = Day22Data {
            secrets: vec![1, 2, 3, 2024],
        };
        let (_, result_part2) = challenge.sum_up_new_secrets(2000);
        println!("result day 22 part 2: {}", result_part2);
        assert_eq!(result_part2, 23);

        Ok(())
    }
}
