//!day_21.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::HashMap;

#[derive(Debug)]
struct KeyPad {
    keys: HashMap<Point, char>,
}

impl KeyPad {
    fn new_num_pad() -> Self {
        let mut keys: HashMap<Point, char> = HashMap::with_capacity(11);
        keys.insert((0, 0).into(), '7');
        keys.insert((1, 0).into(), '8');
        keys.insert((2, 0).into(), '9');
        keys.insert((0, 1).into(), '4');
        keys.insert((1, 1).into(), '5');
        keys.insert((2, 1).into(), '6');
        keys.insert((0, 2).into(), '1');
        keys.insert((1, 2).into(), '2');
        keys.insert((2, 2).into(), '3');
        keys.insert((1, 3).into(), '0');
        keys.insert((2, 3).into(), 'A');
        Self { keys }
    }
    fn new_dir_pad() -> Self {
        let mut keys: HashMap<Point, char> = HashMap::with_capacity(5);
        keys.insert((1, 0).into(), '^');
        keys.insert((2, 0).into(), 'A');
        keys.insert((0, 1).into(), '<');
        keys.insert((1, 1).into(), 'v');
        keys.insert((2, 1).into(), '>');
        Self { keys }
    }
    fn key_strokes(&self, from: char, to: char) -> Vec<String> {
        if from == to {
            return vec!["A".into()];
        }
        let from_pos = self
            .keys
            .iter()
            .find(|(_, v)| **v == from)
            .map(|(k, _)| *k)
            .unwrap();
        let to_pos = self
            .keys
            .iter()
            .find(|(_, v)| **v == to)
            .map(|(k, _)| *k)
            .unwrap();
        self.key_strokes_recursive(from_pos, to_pos)
    }
    fn key_strokes_recursive(&self, from: Point, to: Point) -> Vec<String> {
        if from == to {
            return vec!["A".into()];
        }
        let new_delta = from.delta(to) - 1;
        let mut sequences: Vec<String> = Vec::new();
        for (new_from, dir_char) in [
            (Point::new(0, -1), "^"),
            (Point::new(1, 0), ">"),
            (Point::new(0, 1), "v"),
            (Point::new(-1, 0), "<"),
        ]
        .iter()
        .map(|(p, d)| (p.add(from), d))
        .filter(|(p, _)| self.keys.contains_key(p) && p.delta(to) == new_delta)
        {
            for sub_sequence in self.key_strokes_recursive(new_from, to).iter() {
                let sequence = dir_char.to_string() + sub_sequence;
                sequences.push(sequence);
            }
        }
        sequences
    }
}

#[derive(Debug)]
struct Day21Data {
    codes: Vec<String>,
    dir_robots: usize,
    num_pad: KeyPad,
    dir_pad: KeyPad,
}

impl From<&str> for Day21Data {
    fn from(value: &str) -> Self {
        let codes: Vec<String> = value.lines().map(|l| l.to_owned()).collect();
        Self {
            codes,
            dir_robots: 2,
            num_pad: KeyPad::new_num_pad(),
            dir_pad: KeyPad::new_dir_pad(),
        }
    }
}

impl Day21Data {
    fn calc_complexities(&mut self, dir_robots: usize) -> usize {
        let codes = self.codes.to_owned();
        let mut complexities = 0;
        let mut cache: HashMap<(char, char, usize), usize> = HashMap::new();
        self.dir_robots = dir_robots;
        for code in codes.iter() {
            let sequence_len = self.get_key_pad_sequence(code, &mut cache, 0);
            let num_value: usize = code[..3].parse::<usize>().unwrap();
            complexities += sequence_len * num_value;
        }
        complexities
    }
    fn get_key_pad_sequence(
        &self,
        code: &str,
        cache: &mut HashMap<(char, char, usize), usize>,
        level: usize,
    ) -> usize {
        let mut sequence_len = 0;
        let mut previous_key = 'A';
        for key in code.chars() {
            if let Some(cached_len) = cache.get(&(previous_key, key, level)) {
                sequence_len += cached_len;
                previous_key = key;
                continue;
            }
            let possible_key_strokes = if level == 0 {
                self.num_pad.key_strokes(previous_key, key)
            } else {
                self.dir_pad.key_strokes(previous_key, key)
            };
            let sub_sequence_len = if level == self.dir_robots {
                possible_key_strokes
                    .iter()
                    .map(|ks| ks.len())
                    .min()
                    .unwrap()
            } else {
                let mut min_len = usize::MAX;
                for key_strokes in possible_key_strokes.iter() {
                    let recursive_len = self.get_key_pad_sequence(key_strokes, cache, level + 1);
                    min_len = min_len.min(recursive_len);
                }
                min_len
            };
            sequence_len += sub_sequence_len;
            cache.insert((previous_key, key, level), sub_sequence_len);
            previous_key = key;
        }
        sequence_len
    }
}

pub fn day_21() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_21.txt");
    let mut challenge = Day21Data::from(input);

    let result_part1 = challenge.calc_complexities(2);
    println!("result day 21 part 1: {}", result_part1);
    assert_eq!(result_part1, 197_560);

    let result_part2 = challenge.calc_complexities(25);
    println!("result day 21 part 2: {}", result_part2);
    assert_eq!(result_part2, 242_337_182_910_752);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key_strokes() {
        let num_pad = KeyPad::new_num_pad();
        assert_eq!(num_pad.key_strokes('A', '0'), ["<A"]);
        assert_eq!(num_pad.key_strokes('0', '9').len(), 4);
        assert_eq!(num_pad.key_strokes('1', '0'), [">vA"]);
        assert_eq!(num_pad.key_strokes('0', '1'), ["^<A"]);
        let from_7_to_a = num_pad.key_strokes('7', 'A').len();
        assert_eq!(from_7_to_a, 9);
        assert_eq!(num_pad.key_strokes('A', '7').len(), from_7_to_a);
        assert_eq!(num_pad.key_strokes('0', '0'), ["A"]);

        let dir_pad = KeyPad::new_dir_pad();
        assert_eq!(dir_pad.key_strokes('<', '^'), [">^A"]);
        assert_eq!(dir_pad.key_strokes('^', '<'), ["v<A"]);
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_21_example.txt");
        let mut challenge = Day21Data::from(input);

        let result_part1 = challenge.calc_complexities(2);
        println!("result day 21 part 1: {}", result_part1);
        assert_eq!(result_part1, 126_384);
        /*
        let result_part2 = challenge
        println!("result day 21 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
