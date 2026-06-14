//!day_05.rs

use anyhow::Result;
use std::collections::HashMap;

struct Line<'a> {
    line: &'a str,
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(line: &'a str) -> Self {
        Line { line }
    }
}

impl<'a> Line<'a> {
    fn is_nice(&self) -> bool {
        let mut vowel_count = 0;
        let mut contains_valid_pair = false;
        let mut pair = [' ', ' '];
        for c in self.line.chars() {
            if "aeiou".contains(c) {
                vowel_count += 1;
            }
            pair.rotate_left(1);
            pair[1] = c;
            if pair[0] == pair[1] {
                contains_valid_pair = true;
            }
            match pair {
                ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => return false,
                _ => (),
            }
        }
        contains_valid_pair && vowel_count > 2
    }
    fn is_even_nicer(&self) -> bool {
        let mut seen: HashMap<[char; 2], u16> = HashMap::new();
        let mut three = [' ', ' ', ' '];
        let mut contains_x_x = false;
        for c in self.line.chars() {
            three.rotate_left(1);
            three[2] = c;
            if three[0] != ' ' && three[0] == three[2] {
                contains_x_x = true;
            }
            // only add two chars to seen, if they differ from previous two chars
            if three[1..] != three[..2] {
                seen.entry([three[1], three[2]])
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
        contains_x_x && seen.values().any(|v| *v > 1)
    }
}

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
            .map(Line::from)
            .filter(|l| l.is_nice())
            .count()
    }
    fn solution_part_2(&self) -> usize {
        self.input
            .lines()
            .map(Line::from)
            .filter(|l| l.is_even_nicer())
            .count()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_05.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 258);

    let result_part2 = challenge.solution_part_2();
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 53);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_05_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_05_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, 2);

        Ok(())
    }

    #[test]
    fn test_example_day_05_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_05_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_05 part 2: {result_part2}");
        assert_eq!(result_part2, 2);

        Ok(())
    }
}
