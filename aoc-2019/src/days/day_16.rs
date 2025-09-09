//!day_16.rs

use anyhow::Result;

struct RepeatingPattern {
    repeat: usize,
    count: usize,
    index: usize,
}

impl RepeatingPattern {
    const PATTERN: [i64; 4] = [0, 1, 0, -1];
    fn new(repeat: usize) -> Self {
        RepeatingPattern {
            repeat,
            count: 0,
            index: 0,
        }
    }
}

impl Iterator for RepeatingPattern {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.repeat {
            self.index += 1;
            if self.index == Self::PATTERN.len() {
                self.index = 0;
            }
            self.count = 0;
        } else {
            self.count += 1;
        }
        Some(Self::PATTERN[self.index])
    }
}

struct ChallengeInput {
    numbers: Vec<i64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value
                .chars()
                .filter_map(|d| d.to_digit(10))
                .map(|d| d as i64)
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> i64 {
        for _ in 0..100 {
            self.one_phase_of_fft();
        }
        self.get_first_n_digits(8)
    }
    fn solution_part_2(&mut self) -> i64 {
        let offset = self.get_first_n_digits(7) as usize;
        // part two uses a trick, see comment in fn simple_fft()
        assert!(offset > self.numbers.len() * 10_000 / 2);
        self.extend_n_times_and_cut_off(10_000, offset);
        for _ in 0..100 {
            self.simple_fft();
        }
        self.get_first_n_digits(8)
    }
    fn one_phase_of_fft(&mut self) {
        let mut fft: Vec<i64> = vec![0; self.numbers.len()];
        for (pos, value) in fft.iter_mut().enumerate() {
            let sum: i64 = self
                .numbers
                .iter()
                .zip(RepeatingPattern::new(pos))
                .filter_map(|(&d, p)| match p {
                    1 => Some(d),
                    -1 => Some(-d),
                    _ => None,
                })
                .sum();
            *value = sum.abs() % 10;
        }

        self.numbers = fft;
    }
    fn get_first_n_digits(&self, n: usize) -> i64 {
        let mut out = 0;
        for number in self.numbers.iter().take(n) {
            out *= 10;
            out += number;
        }
        out
    }
    fn extend_n_times_and_cut_off(&mut self, n: usize, offset: usize) {
        let mut extended: Vec<i64> = Vec::with_capacity(self.numbers.len() * n);
        for _ in 0..n {
            extended.extend_from_slice(&self.numbers[..]);
        }
        self.numbers = extended[offset..].to_vec();
    }
    fn simple_fft(&mut self) {
        // when we have a big offset (offset > self.numbers.len() * 10_000 / 2), the repeating
        // pattern is only ones starting at offset until end of vec.
        // Therefore we only need to add up numbers for new value at pos, starting from pos.
        // Even better, if we start from the end, we only need to run once through the iterator
        // by adding up total and getting current %10 digit for each pos.
        let mut total = 0;
        for pos in (0..self.numbers.len()).rev() {
            total += self.numbers[pos];
            self.numbers[pos] = total % 10;
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_16.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, 70_856_418);

    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, 87_766_336);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_repeating_pattern_iter() {
        let pat = RepeatingPattern::new(0);
        let first_five: Vec<i64> = pat.take(5).collect();
        assert_eq!(first_five, [1, 0, -1, 0, 1]);

        let pat = RepeatingPattern::new(1);
        let first_ten: Vec<i64> = pat.take(10).collect();
        assert_eq!(first_ten, [0, 1, 1, 0, 0, -1, -1, 0, 0, 1]);

        let pat = RepeatingPattern::new(2);
        let first_twenty: Vec<i64> = pat.take(20).collect();
        assert_eq!(
            first_twenty,
            [
                0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_example_day_16() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_16_example.txt");
        let solutions = [
            24_176_176, 73_745_418, 52_432_133, 84_462_026, 78_725_270, 53_553_731,
        ];

        for (index, (input, solution)) in multi_input.lines().zip(solutions).enumerate() {
            let mut example = ChallengeInput::from(input);

            if index < 3 {
                let result_part1 = example.solution_part_1();
                println!("result day_16 part 1: {result_part1}");
                assert_eq!(result_part1, solution);
            } else {
                let result_part2 = example.solution_part_2();
                println!("result day_16 part 2: {result_part2}");
                assert_eq!(result_part2, solution);
            }
        }

        Ok(())
    }
}
