//!day_09.rs

use anyhow::Result;

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { input: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> u64 {
        let mut counter: u64 = 0;
        let mut len = 0;
        let mut marker = None::<String>;
        for c in self.input.chars().filter(|c| !c.is_whitespace()) {
            if counter > 0 {
                counter -= 1;
            } else if let Some(m) = &mut marker {
                if c == ')' {
                    let (l, f) = m.split_once('x').unwrap();
                    counter = l.parse().unwrap();
                    let factor = f.parse::<u64>().unwrap();
                    len += counter * factor;
                    marker = None;
                } else {
                    m.push(c);
                }
            } else if c == '(' {
                marker = Some(String::new());
            } else {
                len += 1;
            }
        }
        len
    }
    fn solution_part_2(&self) -> u64 {
        let mut counter: u64 = 0;
        let mut len = 0;
        let mut buffer = None::<String>;
        let mut factor = 0_u64;
        for c in self.input.chars().filter(|c| !c.is_whitespace()) {
            if counter > 0
                && let Some(b) = &mut buffer
            {
                b.push(c);
                counter -= 1;
                if counter == 0 {
                    let sub_challenge = ChallengeInput { input: b };
                    let sub_len = sub_challenge.solution_part_2();
                    len += factor * sub_len;
                    buffer = None;
                }
            } else if let Some(marker) = &mut buffer {
                if c == ')' {
                    let (l, f) = marker.split_once('x').unwrap();
                    counter = l.parse().unwrap();
                    factor = f.parse().unwrap();
                    buffer = Some(String::new());
                } else {
                    marker.push(c);
                }
            } else if c == '(' {
                buffer = Some(String::new());
            } else {
                len += 1;
            }
        }
        len
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 123_908);

    let result_part2 = challenge.solution_part_2();
    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 10_755_693_147);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_09_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_09_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_09 part 1: {result_part1}");
        assert_eq!(result_part1, 6 + 7 + 9 + 11 + 6 + 18);

        Ok(())
    }

    #[test]
    fn test_example_day_09_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_09_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_09 part 2: {result_part2}");
        assert_eq!(result_part2, 9 + 20 + 241_920 + 445);

        Ok(())
    }
}
