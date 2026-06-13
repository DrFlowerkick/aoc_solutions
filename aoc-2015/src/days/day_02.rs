//!day_02.rs

use anyhow::Result;

#[derive(Clone, Copy)]
struct Present {
    l: u64,
    w: u64,
    h: u64,
}

impl From<&str> for Present {
    fn from(value: &str) -> Self {
        let mut value_iter = value.split("x").filter_map(|v| v.parse().ok());
        Present {
            l: value_iter.next().unwrap(),
            w: value_iter.next().unwrap(),
            h: value_iter.next().unwrap(),
        }
    }
}

impl Present {
    fn surface(&self) -> u64 {
        2 * self.l * self.w + 2 * self.l * self.h + 2 * self.w * self.h
    }
    fn min_surface(&self) -> u64 {
        (self.l * self.w).min(self.l * self.h).min(self.w * self.h)
    }
    fn min_circumference(&self) -> u64 {
        let mut sides = [self.l, self.w, self.h];
        sides.sort();
        2 * sides[0] + 2 * sides[1]
    }
    fn volume(&self) -> u64 {
        self.l * self.w * self.h
    }
}

struct ChallengeInput {
    presents: Vec<Present>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            presents: value.lines().map(Present::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.presents
            .iter()
            .map(|p| p.surface() + p.min_surface())
            .sum()
    }
    fn solution_part_2(&self) -> u64 {
        self.presents
            .iter()
            .map(|p| p.min_circumference() + p.volume())
            .sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_02.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_02 part 1: {result_part1}");
    assert_eq!(result_part1, 1_598_415);

    let result_part2 = challenge.solution_part_2();
    println!("result day_02 part 2: {result_part2}");
    assert_eq!(result_part2, 3_812_909);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_02() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_02_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_02 part 1: {result_part1}");
        assert_eq!(result_part1, 58 + 43);

        let result_part2 = example.solution_part_2();
        println!("result day_02 part 2: {result_part2}");
        assert_eq!(result_part2, 34 + 14);

        Ok(())
    }
}
