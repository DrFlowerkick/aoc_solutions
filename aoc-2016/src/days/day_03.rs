//!day_03.rs

use anyhow::Result;

#[derive(Clone, Copy, Default, Debug)]
struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl From<&str> for Triangle {
    fn from(value: &str) -> Self {
        let mut digits = value.split_whitespace().filter_map(|d| d.parse().ok());
        Self {
            a: digits.next().unwrap(),
            b: digits.next().unwrap(),
            c: digits.next().unwrap(),
        }
    }
}

impl Triangle {
    fn is_triangle(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

struct ChallengeInput {
    triangles: Vec<Triangle>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            triangles: value.lines().map(Triangle::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        self.triangles.iter().filter(|t| t.is_triangle()).count()
    }
    fn parse_part_2(input: &str) -> Self {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut three_triangles = [Triangle::default(); 3];
        for (l, line) in input.lines().enumerate() {
            let l = l % 3;
            for (i, digit) in line
                .split_whitespace()
                .filter_map(|d| d.parse::<u64>().ok())
                .enumerate()
            {
                match l {
                    0 => three_triangles[i].a = digit,
                    1 => three_triangles[i].b = digit,
                    2 => three_triangles[i].c = digit,
                    _ => unreachable!(),
                }
            }
            if l == 2 {
                triangles.extend_from_slice(&three_triangles);
            }
        }
        Self { triangles }
    }
    fn solution_part_2(&self) -> usize {
        self.solution_part_1()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_03.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_03 part 1: {result_part1}");
    assert_eq!(result_part1, 982);

    let challenge = ChallengeInput::parse_part_2(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_03 part 2: {result_part2}");
    assert_eq!(result_part2, 1_826);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_03() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_03_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_03 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        let example = ChallengeInput::parse_part_2(input);
        let result_part2 = example.solution_part_2();
        println!("result day_03 part 2: {result_part2}");
        assert_eq!(result_part2, 6);

        Ok(())
    }
}
