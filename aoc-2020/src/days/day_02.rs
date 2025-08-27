//!day_02.rs

use anyhow::Result;

struct Password {
    password: String,
    letter: char,
    min: usize,
    max: usize,
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        let sections: Vec<&str> = value.split_whitespace().collect();
        let (min, max) = sections[0].split_once('-').unwrap();
        let min: usize = min.parse().unwrap();
        let max: usize = max.parse().unwrap();
        assert!(min < max);

        let letter = sections[1].chars().next().unwrap();
        assert!(letter.is_ascii_alphabetic());

        Password {
            password: sections[2].into(),
            letter,
            min,
            max,
        }
    }
}

impl Password {
    fn is_valid_part1(&self) -> bool {
        let letter_count = self.password.chars().filter(|c| *c == self.letter).count();
        letter_count >= self.min && letter_count <= self.max
    }
    fn is_valid_part2(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .fold(0, |count, (index, l)| {
                if (index + 1 == self.min || index + 1 == self.max) && l == self.letter {
                    count + 1
                } else {
                    count
                }
            })
            == 1
    }
}

struct ChallengeInput {
    passwords: Vec<Password>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            passwords: value.lines().map(Password::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.passwords
            .iter()
            .filter(|pw| pw.is_valid_part1())
            .count()
    }
    fn solution_part_2(&self) -> usize {
        self.passwords
            .iter()
            .filter(|pw| pw.is_valid_part2())
            .count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, 548);

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, 502);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_02_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        let result_part2 = example.solution_part_2();
        println!("result day_02 part 2: {result_part2}");
        assert_eq!(result_part2, 1);

        Ok(())
    }
}
