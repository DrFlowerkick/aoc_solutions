//!day_10.rs

use anyhow::Result;
use std::fmt::Write;

struct Ring {
    ring: Vec<u64>,
    current_pos: usize,
    skip_size: usize,
}

impl Ring {
    fn new(size: usize) -> Self {
        Ring {
            ring: (0..size as u64).into_iter().collect(),
            current_pos: 0,
            skip_size: 0,
        }
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
    fn knot_hash(&self) -> String {
        assert_eq!(self.ring.len(), 256);
        let mut dense_hash: Vec<u64> = vec![0; 16];
        for i in 0..16_usize {
            let pos = i * 16;
            let slice = &self.ring[pos..pos + 16];
            dense_hash[i] = slice.iter().fold(0, |mut xor, v| {
                xor ^= v;
                xor
            });
        }
        let mut knot_hash = String::new();
        for dh in dense_hash {
            write!(&mut knot_hash, "{:02x}", dh).unwrap();
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
        let mut ring = Ring::new(size);
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
        let mut ring = Ring::new(256);
        let lengths = self.input_as_ascii_to_len();
        // 64 rounds
        for _ in 0..64 {
            for len in lengths.iter() {
                ring.tying_a_knot(*len);
            }
        }
        ring.knot_hash()
    }
    fn input_as_ascii_to_len(&self) -> Vec<usize> {
        let mut len: Vec<usize> = self.input.chars().map(|c| c as usize).collect();
        // append len suffix
        len.append(&mut vec![17, 31, 73, 47, 23]);
        len
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
