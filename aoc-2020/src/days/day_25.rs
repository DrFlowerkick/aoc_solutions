//!day_25.rs

use anyhow::Result;

struct ChallengeInput {
    card_public_key: u64,
    door_public_key: u64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut keys = value.lines().filter_map(|l| l.parse::<u64>().ok());
        ChallengeInput {
            card_public_key: keys.next().unwrap(),
            door_public_key: keys.next().unwrap(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let card_loop_size = self.get_secret_loop_size_from_public_key(self.card_public_key);
        let door_loop_size = self.get_secret_loop_size_from_public_key(self.door_public_key);
        let encryption_key = self.get_encryption_key(card_loop_size, self.door_public_key);
        assert_eq!(
            self.get_encryption_key(door_loop_size, self.card_public_key),
            encryption_key
        );
        encryption_key
    }
    fn get_secret_loop_size_from_public_key(&self, key: u64) -> u64 {
        let mut value = 1;
        let subject_number = 7;
        let mut loop_size = 0;
        while value != key {
            value *= subject_number;
            value %= 20_201_227;
            loop_size += 1;
        }
        loop_size
    }
    fn get_encryption_key(&self, loop_size: u64, key: u64) -> u64 {
        let mut value = 1;
        for _ in 0..loop_size {
            value *= key;
            value %= 20_201_227;
        }
        value
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_25.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_25 part 1: {result_part1}");
    assert_eq!(result_part1, 297_257);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_25() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_25_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_25 part 1: {result_part1}");
        assert_eq!(result_part1, 14_897_079);

        Ok(())
    }
}
