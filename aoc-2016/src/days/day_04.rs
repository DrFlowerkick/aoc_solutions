//!day_04.rs

use anyhow::Result;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug)]
struct Room {
    name: String,
    sector_id: u64,
    checksum: String,
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        let mut name = String::new();
        let mut sector_id = 0;
        let mut checksum = String::new();

        for splitter in value.split("-") {
            if let Some((left, right)) = splitter.split_once('[') {
                sector_id = left.parse().unwrap();
                checksum = right.strip_suffix(']').unwrap().to_string();
            } else if name.is_empty() {
                name += splitter;
            } else {
                name.push('-');
                name += splitter;
            }
        }

        Room {
            name,
            sector_id,
            checksum,
        }
    }
}

impl Room {
    fn get_id_of_real(&self) -> Option<u64> {
        let mut letter_count: HashMap<char, u16> = HashMap::new();
        for c in self.name.chars().filter(|c| c.is_alphabetic()) {
            letter_count.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut letters: Vec<char> = letter_count.keys().copied().collect();
        letters.sort_by(|a, b| match letter_count.get(b).cmp(&letter_count.get(a)) {
            Ordering::Equal => a.cmp(b),
            cmp => cmp,
        });

        if self
            .checksum
            .chars()
            .enumerate()
            .any(|(i, c)| c != letters[i])
        {
            return None;
        }

        Some(self.sector_id)
    }
    fn decrypt(&self) -> String {
        let mut decrypt = String::new();
        let letters: Vec<char> = (97_u8..=122).map(|d| d as char).collect();
        let rot = (self.sector_id % 26) as usize;
        for c in self.name.chars() {
            if let Some(pos) = letters.iter().position(|l| *l == c) {
                let pos = (pos + rot) % 26;
                decrypt.push(letters[pos]);
            } else {
                decrypt.push(' ');
            }
        }
        decrypt
    }
}

struct ChallengeInput {
    rooms: Vec<Room>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            rooms: value.lines().map(Room::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.rooms.iter().filter_map(|r| r.get_id_of_real()).sum()
    }
    fn solution_part_2(&self) -> u64 {
        for room in self.rooms.iter() {
            if let Some(id) = room.get_id_of_real() && room.decrypt().contains("north") {
                return id
            }
        }
        0
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_04.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 361_724);

    let result_part2 = challenge.solution_part_2();
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 482);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_04() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_04_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, 1_514);

        Ok(())
    }
}
