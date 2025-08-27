//!day_18.rs

use std::{fmt::Display, str::Chars};

use anyhow::Result;

#[derive(Clone)]
enum SnailFishNumber {
    Single(u32),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>),
}

enum Reduce {
    Explode(Option<u32>, Option<u32>),
    Split,
    None,
}

impl Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishNumber::Single(num) => write!(f, "{num}"),
            SnailFishNumber::Pair(left, right) => {
                write!(f, "[{left},{right}]")
            }
        }
    }
}

impl SnailFishNumber {
    fn new(char_iter: &mut Chars) -> Self {
        let c = char_iter.next().unwrap();
        match c {
            '[' => {
                let left = SnailFishNumber::new(char_iter);
                assert_eq!(char_iter.next(), Some(','));
                let right = SnailFishNumber::new(char_iter);
                assert_eq!(char_iter.next(), Some(']'));
                SnailFishNumber::Pair(Box::new(left), Box::new(right))
            }
            d if d.is_ascii_digit() => SnailFishNumber::Single(d.to_digit(10).unwrap()),
            _ => panic!("unexpected char"),
        }
    }
    fn add(self, other: SnailFishNumber) -> SnailFishNumber {
        let mut result = SnailFishNumber::Pair(Box::new(self), Box::new(other));
        while !matches!(result.reduce(), Reduce::None) {}
        result
    }
    fn reduce(&mut self) -> Reduce {
        match self.search_explode(0) {
            Reduce::None => (),
            reduce => return reduce,
        }
        self.search_split()
    }
    fn search_explode(&mut self, depth: usize) -> Reduce {
        match self {
            SnailFishNumber::Single(_) => Reduce::None,
            SnailFishNumber::Pair(left, right) => {
                if depth == 4 {
                    // explode
                    let SnailFishNumber::Single(ld) = left.as_ref() else {
                        panic!("expected SnailFishNumber::Single")
                    };
                    let SnailFishNumber::Single(rd) = right.as_ref() else {
                        panic!("expected SnailFishNumber::Single")
                    };
                    let reduce = Reduce::Explode(Some(*ld), Some(*rd));
                    *self = SnailFishNumber::Single(0);
                    reduce
                } else {
                    match left.search_explode(depth + 1) {
                        Reduce::Explode(lf, rd) => {
                            if let Some(right_digit) = rd {
                                right.explode(Reduce::Explode(None, Some(right_digit)));
                            }
                            return Reduce::Explode(lf, None);
                        }
                        Reduce::Split => return Reduce::Split,
                        Reduce::None => (),
                    }
                    match right.search_explode(depth + 1) {
                        Reduce::Explode(lf, rd) => {
                            if let Some(left_digit) = lf {
                                left.explode(Reduce::Explode(Some(left_digit), None));
                            }
                            Reduce::Explode(None, rd)
                        }
                        reduce => reduce,
                    }
                }
            }
        }
    }
    fn explode(&mut self, explode: Reduce) {
        match explode {
            Reduce::Explode(Some(ld), None) => {
                match self {
                    SnailFishNumber::Single(num) => {
                        *num += ld;
                    }
                    SnailFishNumber::Pair(_, right) => {
                        // if on left side of explosion is a pair, search on right side of pair for Single
                        right.explode(explode);
                    }
                }
            }
            Reduce::Explode(None, Some(rd)) => {
                match self {
                    SnailFishNumber::Single(num) => {
                        *num += rd;
                    }
                    SnailFishNumber::Pair(left, _) => {
                        // if on right side of explosion is a pair, search on left side of pair for Single
                        left.explode(explode);
                    }
                }
            }
            _ => unreachable!("explode should only be called with either left or right digit."),
        }
    }
    fn search_split(&mut self) -> Reduce {
        match self {
            SnailFishNumber::Single(num) => {
                if *num > 9 {
                    let value = *num / 2;
                    let reminder = *num % 2;
                    let left = SnailFishNumber::Single(value);
                    let right = SnailFishNumber::Single(value + reminder);
                    *self = SnailFishNumber::Pair(Box::new(left), Box::new(right));
                    Reduce::Split
                } else {
                    Reduce::None
                }
            }
            SnailFishNumber::Pair(left, right) => {
                match left.search_split() {
                    Reduce::Explode(_, _) => unreachable!("first do all explode action"),
                    Reduce::Split => return Reduce::Split,
                    Reduce::None => (),
                }
                match right.search_split() {
                    Reduce::Explode(_, _) => unreachable!("first do all explode action"),
                    reduce => reduce,
                }
            }
        }
    }
    fn magnitude(&self) -> u32 {
        match self {
            SnailFishNumber::Single(num) => *num,
            SnailFishNumber::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

struct ChallengeInput {
    snailfish_numbers: Vec<SnailFishNumber>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut snailfish_numbers = Vec::new();
        for line in value.lines() {
            let mut char_iter = line.chars();
            snailfish_numbers.push(SnailFishNumber::new(&mut char_iter));
        }
        ChallengeInput { snailfish_numbers }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> u32 {
        let mut snail_fish_iter = self.snailfish_numbers.iter();
        let mut snail_fish = snail_fish_iter.next().unwrap().clone();
        for next_snail_fish in snail_fish_iter {
            snail_fish = snail_fish.add(next_snail_fish.clone());
        }
        snail_fish.magnitude()
    }
    fn solution_part_2(&self) -> u32 {
        let mut max_magnitude = u32::MIN;
        for (index, first) in self.snailfish_numbers.iter().cloned().enumerate() {
            for second in self.snailfish_numbers.iter().skip(index + 1).cloned() {
                let first_second = first.clone().add(second.clone());
                max_magnitude = max_magnitude.max(first_second.magnitude());
                max_magnitude = max_magnitude.max(second.add(first.clone()).magnitude());
            }
        }
        max_magnitude
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_18.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_18 part 1: {result_part1}");
    assert_eq!(result_part1, 2_541);

    let result_part2 = challenge.solution_part_2();
    println!("result day_18 part 2: {result_part2}");
    assert_eq!(result_part2, 4_647);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reduce() {
        let left = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let mut left_iter = left.chars();
        let left = SnailFishNumber::new(&mut left_iter);
        let right = "[1,1]";
        let mut right_iter = right.chars();
        let right = SnailFishNumber::new(&mut right_iter);
        let mut added = SnailFishNumber::Pair(Box::new(left), Box::new(right));
        println!("{added}");
        assert!(matches!(added.reduce(), Reduce::Explode(_, _)));
        println!("{added}");
        assert!(matches!(added.reduce(), Reduce::Explode(_, _)));
        println!("{added}");
        assert!(matches!(added.reduce(), Reduce::Split));
        println!("{added}");
        assert!(matches!(added.reduce(), Reduce::Split));
        println!("{added}");
        assert!(matches!(added.reduce(), Reduce::Explode(_, _)));
        println!("{added}");
        assert!(matches!(added.reduce(), Reduce::None));
        println!("{added}");

        println!("Test fn add()");
        let left = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let mut left_iter = left.chars();
        let left = SnailFishNumber::new(&mut left_iter);
        let right = "[1,1]";
        let mut right_iter = right.chars();
        let right = SnailFishNumber::new(&mut right_iter);
        let added = left.add(right);
        println!("{added}");
    }

    #[test]
    fn test_example_1() {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_18_example_1.txt");
        let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
        let mut first_iter = lines[0].chars();
        let mut snail_fish = SnailFishNumber::new(&mut first_iter);
        let mut loop_counter = 0;
        println!("{loop_counter}:{snail_fish}");
        for next_snail in lines[1..].iter() {
            loop_counter += 1;
            let mut next_snail_iter = next_snail.chars();
            let next_snail = SnailFishNumber::new(&mut next_snail_iter);
            println!("{next_snail}");
            snail_fish = snail_fish.add(next_snail);
            println!("{loop_counter}:{snail_fish}");
        }
        assert_eq!(snail_fish.magnitude(), 3_488);
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_18_example_2.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_18 part 1: {result_part1}");
        assert_eq!(result_part1, 4_140);

        let result_part2 = example.solution_part_2();
        println!("result day_18 part 2: {result_part2}");
        assert_eq!(result_part2, 3_993);

        Ok(())
    }
}
