//!day_07.rs

use anyhow::Result;
use std::collections::HashSet;

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { input: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> usize {
        self.input
            .lines()
            .filter(|l| {
                let mut buffer = [' '; 4];
                let mut abba = false;
                let mut block_abba = false;
                let mut hyper = 0;
                for c in l.chars() {
                    match c {
                        '[' => {
                            hyper += 1;
                            buffer = [' '; 4];
                        }
                        ']' => {
                            hyper -= 1;
                            buffer = [' '; 4];
                        }
                        l if c.is_alphabetic() => {
                            buffer.rotate_right(1);
                            buffer[0] = l;
                            if buffer[0] == buffer[3]
                                && buffer[1] == buffer[2]
                                && buffer[0] != buffer[1]
                            {
                                if hyper > 0 {
                                    block_abba = true;
                                } else {
                                    abba = true;
                                }
                            }
                        }
                        _ => panic!("unexpected char"),
                    }
                }
                abba && !block_abba
            })
            .count()
    }
    fn solution_part_2(&self) -> usize {
        self.input
            .lines()
            .filter(|l| {
                let mut buffer = [' '; 3];
                let mut aba: HashSet<[char; 3]> = HashSet::new();
                let mut bab: HashSet<[char; 3]> = HashSet::new();
                let mut hyper = 0;
                for c in l.chars() {
                    match c {
                        '[' => {
                            hyper += 1;
                            buffer = [' '; 3];
                        }
                        ']' => {
                            hyper -= 1;
                            buffer = [' '; 3];
                        }
                        l if c.is_alphabetic() => {
                            buffer.rotate_right(1);
                            buffer[0] = l;
                            if buffer[0] == buffer[2] && buffer[0] != buffer[1] {
                                if hyper > 0 {
                                    let inverted = [buffer[1], buffer[0], buffer[1]];
                                    bab.insert(inverted);
                                } else {
                                    aba.insert(buffer);
                                }
                            }
                        }
                        _ => panic!("unexpected char"),
                    }
                }
                aba.intersection(&bab).count() > 0
            })
            .count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_07.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, 118);

    let result_part2 = challenge.solution_part_2();
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 260);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_07_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_07 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        Ok(())
    }

    #[test]
    fn test_example_day_07_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_07_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_07 part 2: {result_part2}");
        assert_eq!(result_part2, 3);

        Ok(())
    }
}
