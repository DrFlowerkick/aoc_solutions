//!day_15.rs

use anyhow::Result;
use std::collections::HashMap;

struct AoCHash {
    // use u64 even if hash is always 0 to 255, because you want to ad upp hash results
    hash: u64,
}

impl From<&str> for AoCHash {
    fn from(value: &str) -> Self {
        let mut result = AoCHash { hash: 0 };
        for c in value.chars() {
            if !c.is_ascii() {
                panic!("bad AoCHash input");
            }
            result.hash += c as u64;
            result.hash *= 17;
            result.hash %= 256;
        }
        result
    }
}

// maximum number of boxes is eqal to maximum number of different lenses
// the real

struct Lens {
    label: String,
    focal_length: u64,
}

impl From<(String, u64)> for Lens {
    fn from(value: (String, u64)) -> Self {
        Lens {
            label: value.0,
            focal_length: value.1,
        }
    }
}

pub fn day_15() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_15.txt");
    let result_part1: u64 = input.split(',').map(|c| AoCHash::from(c).hash).sum();
    println!("result day 15 part 1: {}", result_part1);
    assert_eq!(result_part1, 512_283);

    // part 2
    let mut lens_label_box_cache: HashMap<u64, Vec<Lens>> = HashMap::new();
    for instruction in input.split(',') {
        if instruction.contains('-') {
            // remove lens from box, if it exists
            let label = instruction.split_once('-').unwrap().0.to_string();
            let key = AoCHash::from(label.as_str()).hash;
            if let Some(lenses) = lens_label_box_cache.get_mut(&key)
                && let Some(index) = lenses.iter().position(|l| l.label == label)
            {
                lenses.remove(index);
            }
        } else if instruction.contains('=') {
            // add lens to box
            let lens = Lens::from(
                instruction
                    .split_once('=')
                    .map(|(l, f)| (l.to_string(), f.parse::<u64>().expect("bad instruction")))
                    .unwrap(),
            );
            let key = AoCHash::from(lens.label.as_str()).hash;
            match lens_label_box_cache.get_mut(&key) {
                Some(lenses) => match lenses.iter().position(|l| l.label == lens.label) {
                    Some(pos) => lenses[pos] = lens,
                    None => lenses.push(lens),
                },
                None => {
                    lens_label_box_cache.insert(key, vec![lens]);
                }
            }
        } else {
            panic!("bad instruction");
        }
    }
    let mut result_part2: u64 = 0;
    for (key, lenses) in lens_label_box_cache.iter() {
        result_part2 += (key + 1)
            * lenses
                .iter()
                .enumerate()
                .map(|(i, l)| ((i + 1) as u64) * l.focal_length)
                .sum::<u64>();
    }
    println!("result day 15 part 2: {}", result_part2);
    assert_eq!(result_part2, 215_827);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ascii_code_as_u64() {
        let chars = "HASH";
        let mut char_codes = chars.chars().map(|c| c as u64);
        assert_eq!(char_codes.next().unwrap(), 72);
        assert_eq!(char_codes.next().unwrap(), 65);
        assert_eq!(char_codes.next().unwrap(), 83);
        assert_eq!(char_codes.next().unwrap(), 72);
        assert_eq!(AoCHash::from(chars).hash, 52);

        assert_eq!(AoCHash::from("rn").hash, 0);
        assert_eq!(AoCHash::from("qp").hash, 1);
        assert_eq!(AoCHash::from("cm").hash, 0);
        assert_eq!(AoCHash::from("pc").hash, 3);
        assert_eq!(AoCHash::from("ot").hash, 3);
        assert_eq!(AoCHash::from("ab").hash, 3);
    }
}
