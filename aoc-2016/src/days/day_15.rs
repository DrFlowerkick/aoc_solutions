//!day_15.rs

use anyhow::Result;
use my_lib::my_algo_collection::modinv;

#[derive(Clone, Copy)]
struct Disc {
    num: i64,
    len: i64,
    pos: i64,
}

impl From<&str> for Disc {
    fn from(value: &str) -> Self {
        let value = value.strip_prefix("Disc #").unwrap();
        let value = value.strip_suffix(".").unwrap();
        let (num, rem) = value.split_once(" has ").unwrap();
        let (len, pos) = rem
            .split_once(" positions; at time=0, it is at position ")
            .unwrap();
        Disc {
            num: num.parse().unwrap(),
            len: len.parse().unwrap(),
            pos: pos.parse().unwrap(),
        }
    }
}

impl Disc {
    fn solve_mod_inverse_with_pro_m(&self, prod_m: i64) -> i64 {
        // solve t of equation (pos + (t + num)) mod len == 0 for all discs

        // first transform equation to t = -(pos + num) mod len
        let rem = (-self.pos - self.num).rem_euclid(self.len);

        // prod_m is product of len of each disc
        let prod_m_self = prod_m / self.len;
        // calc mod inverse of prod_m_self mod self.len = rem
        let y = modinv(prod_m_self.rem_euclid(self.len), self.len).unwrap();
        rem * prod_m_self * y
    }
}

struct ChallengeInput {
    discs: Vec<Disc>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            discs: value.lines().map(Disc::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        // normally you should check, that all mod divisor are prims, but I skip it
        let prod_m: i64 = self.discs.iter().map(|d| d.len).product();
        self.discs
            .iter()
            .map(|d| d.solve_mod_inverse_with_pro_m(prod_m))
            .sum::<i64>()
            .rem_euclid(prod_m)
    }
    fn solution_part_2(&mut self) -> i64 {
        let new_disc = Disc {
            num: self.discs.iter().map(|d| d.num).max().unwrap() + 1,
            len: 11,
            pos: 0,
        };
        self.discs.push(new_disc);
        self.solution_part_1()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_15.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 376_777);

    let result_part2 = challenge.solution_part_2();
    println!("result day_15 part 2: {result_part2}");
    assert_eq!(result_part2, 3_903_937);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_15() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_15_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_15 part 1: {result_part1}");
        assert_eq!(result_part1, 5);

        Ok(())
    }
}
