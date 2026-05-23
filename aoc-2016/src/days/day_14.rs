//!day_14.rs

use anyhow::Result;
use md5::{Digest, compute};
use rayon::prelude::*;
use std::collections::BTreeMap;

const fn u8_to_hex(byte: u8) -> [u8; 2] {
    let low = byte & 0x0f;
    let high = byte >> 4;
    [high, low]
}

fn digest_to_hex(bytes: Digest, hex_bytes: &mut [u8; 32]) {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    for (i, &byte) in bytes.iter().enumerate() {
        hex_bytes[i * 2] = HEX_CHARS[(byte >> 4) as usize];
        hex_bytes[i * 2 + 1] = HEX_CHARS[(byte & 0x0f) as usize];
    }
}

struct ChallengeInput<'a> {
    seed: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { seed: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u64 {
        self.search_64th_key(0)
    }
    fn solution_part_2(&self) -> u64 {
        self.search_64th_key(2016)
    }
    fn search_64th_key(&self, stretch_hash: u64) -> u64 {
        let chunk_size = 25_000;
        let hits: BTreeMap<u64, (Digest, u8)> = (0..chunk_size)
            .into_par_iter()
            .filter_map(|index| self.check_triple_md5(index, stretch_hash))
            .collect();

        // check hits from previous run
        hits.iter()
            .enumerate()
            .filter_map(|(i, (t_index, (_, hex)))| {
                hits.iter()
                    .skip(i + 1)
                    .any(|(q_index, (maybe_quintet, _))| {
                        *q_index <= t_index + 1_000 && self.check_quintet_md5(*maybe_quintet, *hex)
                    })
                    .then_some(*t_index)
            })
            .nth(63)
            .unwrap()
    }
    fn check_triple_md5(&self, index: u64, stretch_hash: u64) -> Option<(u64, (Digest, u8))> {
        let mut data = compute(format!("{}{}", self.seed, index));
        let mut hex_bytes = [0_u8; 32];
        for _ in 0..stretch_hash {
            digest_to_hex(data, &mut hex_bytes);
            data = compute(&hex_bytes[..]);
        }
        let mut hex: u8 = 0;
        let mut count: u8 = 0;
        for current in data.iter().flat_map(|b| u8_to_hex(*b).into_iter()) {
            if count == 0 || hex != current {
                hex = current;
                count = 1;
            } else {
                count += 1;
                if count == 3 {
                    return Some((index, (data, hex)));
                }
            }
        }
        None
    }
    fn check_quintet_md5(&self, data: Digest, hex: u8) -> bool {
        let mut count: u8 = 0;
        for current in data.iter().flat_map(|b| u8_to_hex(*b).into_iter()) {
            if current == hex {
                count += 1;
                if count == 5 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
        false
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_14.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, 23_890);

    let result_part2 = challenge.solution_part_2();
    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 22_696);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hex_conversion() {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_14_example.txt");
        let example = ChallengeInput::from(input);

        let index = 18;
        let test = format!("{}{}", example.seed, index);
        let data = compute(test);
        let hex: String = data
            .iter()
            .flat_map(|b| u8_to_hex(*b).into_iter())
            .map(|h| format!("{:x}", h))
            .collect();
        assert_eq!(format!("{:x}", data), hex);

        // part 2: multi stretch
        let index = 0;
        let test = format!("{}{}", example.seed, index);
        let mut data = compute(test);
        let mut hex_bytes = [0_u8; 32];
        assert_eq!(format!("{:x}", data), "577571be4de9dcce85a041ba0410f29f");
        digest_to_hex(data, &mut hex_bytes);
        assert_eq!(
            hex_bytes.iter().map(|h| *h as char).collect::<String>(),
            "577571be4de9dcce85a041ba0410f29f"
        );
        digest_to_hex(data, &mut hex_bytes);
        data = compute(&hex_bytes[..]);
        assert_eq!(format!("{:x}", data), "eec80a0c92dc8a0777c619d9bb51e910");
        for _ in 0..2015 {
            digest_to_hex(data, &mut hex_bytes);
            data = compute(&hex_bytes[..]);
        }
        assert_eq!(format!("{:x}", data), "a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn test_example_day_14_first_key() {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_14_example.txt");
        let example = ChallengeInput::from(input);

        // part 1
        let (index, (data, hex)) = example.check_triple_md5(39, 0).unwrap();
        println!("{:x}", data);
        assert_eq!(index, 39);
        assert_eq!(format!("{:x}", hex), "e");
        assert!(format!("{:x}", data).contains("eee"));

        let (index, (quintet, _)) = example.check_triple_md5(816, 0).unwrap();
        println!("{:x}", quintet);
        assert_eq!(index, 816);
        assert!(example.check_quintet_md5(quintet, hex));
        assert!(format!("{:x}", quintet).contains("eeeee"));

        // part 2
        let (index, (data, hex)) = example.check_triple_md5(10, 2016).unwrap();
        println!("{:x}", data);
        assert_eq!(index, 10);
        assert_eq!(format!("{:x}", hex), "e");
        assert!(format!("{:x}", data).contains("eee"));

        let (index, (quintet, _)) = example.check_triple_md5(89, 2016).unwrap();
        println!("{:x}", quintet);
        assert_eq!(index, 89);
        assert!(example.check_quintet_md5(quintet, hex));
        assert!(format!("{:x}", quintet).contains("eeeee"));
    }

    #[test]
    fn test_example_day_14() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_14_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_14 part 1: {result_part1}");
        assert_eq!(result_part1, 22_728);

        let result_part2 = example.solution_part_2();
        println!("result day_14 part 2: {result_part2}");
        assert_eq!(result_part2, 22_551);

        Ok(())
    }
}
