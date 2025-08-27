//!day_08.rs

use anyhow::Result;

struct Pattern {
    bit_mask: u8,
    number_of_segments: u8,
    digit: Option<u64>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let number_of_segments = value.chars().count() as u8;
        let digit = match number_of_segments {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };
        let bit_mask = value
            .chars()
            .map(|c| match c {
                'a' => 0b0000001,
                'b' => 0b0000010,
                'c' => 0b0000100,
                'd' => 0b0001000,
                'e' => 0b0010000,
                'f' => 0b0100000,
                'g' => 0b1000000,
                _ => panic!("unknown char"),
            })
            .sum();
        Pattern {
            bit_mask,
            number_of_segments,
            digit,
        }
    }
}

struct DigitDisplay {
    patterns: Vec<Pattern>,
    digits: Vec<Pattern>,
}

impl From<&str> for DigitDisplay {
    fn from(value: &str) -> Self {
        let Some((patterns, digits)) = value.split_once(" | ") else {
            panic!("unexpected input")
        };
        DigitDisplay {
            patterns: patterns.split_whitespace().map(Pattern::from).collect(),
            digits: digits.split_whitespace().map(Pattern::from).collect(),
        }
    }
}

impl DigitDisplay {
    fn analyze_patterns(&mut self) {
        // 1. get common bits of patterns with 6 segments
        let common_bits_6_segments = self
            .patterns
            .iter()
            .filter(|p| p.number_of_segments == 6)
            .map(|p| p.bit_mask)
            .fold(0b1111111, |bits, bit_mask| bits & bit_mask);
        // 2. get common bit of digit 1 and common_bits_6_segments -> f
        let bit_mask_1 = self
            .patterns
            .iter()
            .find(|p| p.number_of_segments == 2)
            .unwrap()
            .bit_mask;
        let bit_mask_f = bit_mask_1 & common_bits_6_segments;
        // 3. bit_mask_1 xor bit_mask_c gives us c
        let bit_mask_c = bit_mask_1 ^ bit_mask_f;
        // 4. get position of 6, which is the only 6 segment bit_mask, which does not contain bit_mask_c
        let position_6 = self
            .patterns
            .iter()
            .position(|p| p.number_of_segments == 6 && p.bit_mask & bit_mask_c == 0)
            .unwrap();
        self.patterns[position_6].digit = Some(6);
        // 5. get position of 9, which is the only 6 segment bit_mask, which contains bit_mask_4
        let bit_mask_4 = self
            .patterns
            .iter()
            .find(|p| p.number_of_segments == 4)
            .unwrap()
            .bit_mask;
        let position_9 = self
            .patterns
            .iter()
            .position(|p| p.number_of_segments == 6 && p.bit_mask & bit_mask_4 == bit_mask_4)
            .unwrap();
        self.patterns[position_9].digit = Some(9);
        // 6. get index of 0, which is the remaining 6 segment element with None as digit
        let position_0 = self
            .patterns
            .iter()
            .position(|p| p.number_of_segments == 6 && p.digit.is_none())
            .unwrap();
        self.patterns[position_0].digit = Some(0);
        // 7. get index of 5, which is the only 5 segment bit_mask, which contains common_bits_6_segments
        let position_5 = self
            .patterns
            .iter()
            .position(|p| {
                p.number_of_segments == 5
                    && p.bit_mask & common_bits_6_segments == common_bits_6_segments
            })
            .unwrap();
        self.patterns[position_5].digit = Some(5);
        // 8. get index of 3, which is the only 5 segment bit_mask, which contains bit_mask_1
        let position_3 = self
            .patterns
            .iter()
            .position(|p| p.number_of_segments == 5 && p.bit_mask & bit_mask_1 == bit_mask_1)
            .unwrap();
        self.patterns[position_3].digit = Some(3);
        // 9. get index of 2, which is the remaining 5 segment element with None as digit
        let position_2 = self
            .patterns
            .iter()
            .position(|p| p.number_of_segments == 5 && p.digit.is_none())
            .unwrap();
        self.patterns[position_2].digit = Some(2);
        assert!(self.patterns.iter().all(|p| p.digit.is_some()));

        // set digits of digits
        for index in 0..self.digits.len() {
            let bit_mask = self.digits[index].bit_mask;
            let digit = self
                .patterns
                .iter()
                .find(|p| p.bit_mask == bit_mask)
                .unwrap()
                .digit;
            self.digits[index].digit = digit;
        }
    }

    fn display_value(&self) -> Option<u64> {
        if self.digits.iter().any(|p| p.digit.is_none()) {
            return None;
        }
        let initial_exponent = self.digits.len();
        let value = self
            .digits
            .iter()
            .filter_map(|p| p.digit)
            .fold(
                (0, initial_exponent as u32),
                |(mut value, exponent), digit| {
                    value += digit * 10_u64.pow(exponent - 1);
                    (value, exponent - 1)
                },
            )
            .0;
        Some(value)
    }
}

struct Day08Input {
    displays: Vec<DigitDisplay>,
}

impl From<&str> for Day08Input {
    fn from(value: &str) -> Self {
        Day08Input {
            displays: value.lines().map(DigitDisplay::from).collect(),
        }
    }
}

impl Day08Input {
    fn count_simple_patterns(&self) -> usize {
        self.displays
            .iter()
            .flat_map(|dd| dd.digits.iter())
            .filter_map(|p| p.digit)
            .count()
    }
    fn sum_values(&mut self) -> u64 {
        self.displays.iter_mut().for_each(|d| d.analyze_patterns());
        self.displays.iter().filter_map(|d| d.display_value()).sum()
    }
}

pub fn day_08() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_08.txt");
    let mut input = Day08Input::from(input);

    let result_part1 = input.count_simple_patterns();
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 278);

    let result_part2 = input.sum_values();
    println!("result day_08 part 2: {result_part2}");
    assert_eq!(result_part2, 986_179);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_08_example.txt");
        let mut input = Day08Input::from(input);

        let result_part1 = input.count_simple_patterns();
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 26);

        let result_part2 = input.sum_values();
        println!("result day_08 part 2: {result_part2}");
        assert_eq!(result_part2, 61_229);

        Ok(())
    }
}
