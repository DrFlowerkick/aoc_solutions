//!day_11.rs

use anyhow::Result;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct ChallengeInput {
    pw: [u8; 8],
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut pw = [0; 8];
        for (i, c) in value.chars().enumerate() {
            pw[i] = c as u8;
        }
        ChallengeInput { pw }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> String {
        let mut pw = *self;
        while !pw.is_valid() {
            pw.inc();
        }
        pw.as_string()
    }
    fn inc(&mut self) {
        let a = b'a';
        let z = b'z';
        for i in (0..self.pw.len()).rev() {
            if self.pw[i] == z {
                self.pw[i] = a;
            } else {
                self.pw[i] += 1;
                break;
            }
        }
    }
    fn is_valid(&self) -> bool {
        let i = b'i';
        let o = b'o';
        let l = b'l';
        let mut two = [0_u8; 2];
        let mut seen_two: HashSet<[u8; 2]> = HashSet::new();
        let mut three = [0_u8; 3];
        let mut has_three = false;

        for c in self.pw.iter() {
            if *c == i || *c == o || *c == l {
                return false;
            }
            two.rotate_left(1);
            two[1] = *c;
            if two[0] == two[1] {
                seen_two.insert(two);
            }
            three.rotate_left(1);
            three[2] = *c;
            has_three |= three[0] + 2 == three[2] && three[1] + 1 == three[2];
        }

        has_three && seen_two.len() > 1
    }
    fn as_string(&self) -> String {
        self.pw.into_iter().map(|c| c as char).collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, "vzbxxyzz");

    let mut challenge = ChallengeInput::from(result_part1.as_str());
    challenge.inc();
    let result_part2 = challenge.solution_part_1();
    println!("result day_11 part 2: {result_part2}");
    assert_eq!(result_part2, "vzcaabcc");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inc() {
        let mut test_01 = ChallengeInput::from("aaaaaaaz");
        test_01.inc();
        assert_eq!(test_01.as_string(), "aaaaaaba");

        let mut test_02 = ChallengeInput::from("aaaaaazz");
        test_02.inc();
        assert_eq!(test_02.as_string(), "aaaaabaa");

        let mut test_03 = ChallengeInput::from("aaaaaazy");
        test_03.inc();
        assert_eq!(test_03.as_string(), "aaaaaazz");

        let mut test_04 = ChallengeInput::from("abcdfezz");
        test_04.inc();
        assert_eq!(test_04.as_string(), "abcdffaa");
    }

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_11_example.txt");

        let solutions = ["abcdffaa", "ghjaabcc"];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_11 part 1: {result_part1}");
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }
}
