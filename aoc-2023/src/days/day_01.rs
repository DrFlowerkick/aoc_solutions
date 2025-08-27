//!day_01.rs

use anyhow::Result;

pub fn day_01() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_01.txt");
    part_1(input)?;
    part_2(input)
}

fn part_1(input: &str) -> Result<()> {
    let sum: u32 = input
        .lines()
        .map(|l| match l.chars().find(|c| c.is_ascii_digit()) {
            Some(left) => {
                let right = l.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
                let mut number = String::new();
                number.push(left);
                number.push(right);
                number.parse::<u32>().unwrap()
            }
            None => 0,
        })
        .sum();
    println!("result day 01 part 1: {}", sum);
    assert_eq!(sum, 56_465);
    Ok(())
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const CHARS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn part_2(input: &str) -> Result<()> {
    let sum: u32 = input
        .lines()
        .map(|l| {
            let mut index = 0;
            let left = loop {
                if l[index..].chars().next().unwrap().is_ascii_digit() {
                    break l[index..].chars().next().unwrap();
                }
                match DIGITS
                    .into_iter()
                    .enumerate()
                    .filter(|(_, d)| l[index..].starts_with(d))
                    .map(|(i, _)| CHARS[i])
                    .next()
                {
                    Some(d) => break d,
                    None => index += 1,
                }
            };
            let mut len = l[..].len();
            let right = loop {
                if l[..len].chars().last().unwrap().is_ascii_digit() {
                    break l[..len].chars().last().unwrap();
                }
                match DIGITS
                    .into_iter()
                    .enumerate()
                    .filter(|(_, d)| l[..len].ends_with(d))
                    .map(|(i, _)| CHARS[i])
                    .next()
                {
                    Some(d) => break d,
                    None => len -= 1,
                }
            };
            let mut number = String::new();
            number.push(left);
            number.push(right);
            number
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("could not parse {}{}", left, right))
        })
        .sum();
    println!("result day 01 part 2: {}", sum);
    assert_eq!(sum, 55_902);
    Ok(())
}
