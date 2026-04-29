//!day_10.rs

use anyhow::Result;

pub struct KnotHash {
    ring: Vec<u64>,
    current_pos: usize,
    skip_size: usize,
}

impl Default for KnotHash {
    fn default() -> Self {
        Self::new()
    }
}

impl KnotHash {
    pub fn new() -> Self {
        KnotHash {
            ring: (0..256).collect(),
            current_pos: 0,
            skip_size: 0,
        }
    }
    fn strip_ring(&mut self, reduced_size: usize) {
        // this is required for example 1, which uses a size of 5
        self.ring.truncate(reduced_size);
    }
    fn tying_a_knot(&mut self, len: usize) {
        assert!(len <= self.ring.len());
        let mut rotation = 0;
        if self.current_pos + len > self.ring.len() {
            rotation = self.current_pos + len - self.ring.len();
        }
        self.ring.rotate_left(rotation);
        let post_rotate_pos = self.current_pos - rotation;
        let slice = &mut self.ring[post_rotate_pos..post_rotate_pos + len];
        slice.reverse();
        self.ring.rotate_right(rotation);
        self.current_pos = (self.current_pos + len + self.skip_size).rem_euclid(self.ring.len());
        self.skip_size += 1;
    }
    fn check(&self) -> u64 {
        self.ring[0] * self.ring[1]
    }
    pub fn knot_hash(mut self, input: impl Into<String>) -> u128 {
        // consumes self, because ring buffer represents hash state
        assert_eq!(self.ring.len(), 256);
        // convert input to len vector
        let input: String = input.into();
        let mut lengths: Vec<usize> = input.chars().map(|c| c as usize).collect();
        // append len suffix
        lengths.append(&mut vec![17, 31, 73, 47, 23]);

        // transform ring in 64 rounds
        for _ in 0..64 {
            for len in lengths.iter() {
                self.tying_a_knot(*len);
            }
        }

        // convert ring to dense hash
        let mut knot_hash = 0_u128;
        for i in 0..16 {
            let pos = i * 16;
            let slice = &self.ring[pos..pos + 16];
            let dense_bits = slice.iter().fold(0, |mut xor, v| {
                xor ^= *v as u128;
                xor
            });
            knot_hash = knot_hash.rotate_left(8);
            knot_hash += dense_bits;
        }

        knot_hash
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
    fn solution_part_1(&self, size: usize) -> u64 {
        let mut ring = KnotHash::new();
        ring.strip_ring(size);
        let lengths = self.input_to_len();
        for len in lengths {
            ring.tying_a_knot(len);
        }
        ring.check()
    }
    fn input_to_len(&self) -> Vec<usize> {
        self.input
            .split(",")
            .filter_map(|l| l.parse().ok())
            .collect()
    }
    fn solution_part_2(&self) -> String {
        let ring = KnotHash::new();
        format!("{:032x}", ring.knot_hash(self.input))
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_10.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(256);
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 1_935);

    let result_part2 = challenge.solution_part_2();
    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, "dc7e7dee710d4c7201ce42713e6b8359");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_10_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_10_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(5);
        println!("result day_10 part 1: {result_part1}");
        assert_eq!(result_part1, 12);

        Ok(())
    }

    #[test]
    fn test_example_day_10_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_10_example_part_2.txt");

        let solutions = [
            "a2582a3a0e66e6e86e3812dcb672a272",
            "33efeb34ea91902bb2f59c9920caa6cd",
            "3efbe78a8d82f29979031a4aa0b16a9d",
            "63960835bcdc130f0b66d7ff4f6a5a8e",
        ];

        for (line, solution) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part2 = example.solution_part_2();
            println!("result day_10 part 2: {result_part2}");
            assert_eq!(result_part2, solution);
        }

        Ok(())
    }
}
