//!day_20.rs

use anyhow::Result;
use fixedbitset::FixedBitSet;
use my_lib::my_geometry::my_point::Point;
use std::collections::HashSet;

#[derive(Clone)]
struct Image {
    pixels: HashSet<Point>,
    top_left: Point,
    bottom_right: Point,
    outside_light: bool,
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        Image {
            pixels: value
                .lines()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .filter_map(move |(x, c)| (c == '#').then_some((x as i64, y as i64).into()))
                })
                .collect(),
            top_left: (0, 0).into(),
            bottom_right: {
                let x = value.lines().next().unwrap().chars().count() - 1;
                let y = value.lines().count() - 1;
                (x as i64, y as i64)
            }
            .into(),
            outside_light: false,
        }
    }
}

impl Image {
    fn generate_child(&self, algo: &FixedBitSet) -> Self {
        let mut child = Image {
            pixels: HashSet::new(),
            top_left: self.top_left.add((-1, -1).into()),
            bottom_right: self.bottom_right.add((1, 1).into()),
            outside_light: self.outside_light ^ algo[0],
        };
        for y in child.top_left.y..=child.bottom_right.y {
            for x in child.top_left.x..=child.bottom_right.x {
                let pixel = Point::new(x, y);
                if self.get_new_pixel_value(algo, pixel) {
                    child.pixels.insert(pixel);
                }
            }
        }
        child
    }
    fn get_new_pixel_value(&self, algo: &FixedBitSet, pixel: Point) -> bool {
        let mut index = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                index <<= 1;
                let new_pixel = pixel.add((dx, dy).into());
                let outside_value = (new_pixel.x < self.top_left.x
                    || new_pixel.x > self.bottom_right.x
                    || new_pixel.y < self.top_left.y
                    || new_pixel.y > self.bottom_right.y)
                    && self.outside_light;
                if self.pixels.contains(&new_pixel) || outside_value {
                    index += 1;
                }
            }
        }
        algo[index]
    }
}

struct ChallengeInput {
    algo: FixedBitSet,
    image: Image,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (algo_bits, image) = value.split_once("\n\n").unwrap();
        let mut algo = FixedBitSet::from_iter(
            algo_bits
                .chars()
                .enumerate()
                .filter_map(|(bit, c)| if c == '#' { Some(bit) } else { None }),
        );
        algo.grow(512);
        let image = image.into();
        ChallengeInput { algo, image }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        let first_image = self.image.generate_child(&self.algo);
        let second_image = first_image.generate_child(&self.algo);
        second_image.pixels.len()
    }
    fn solution_part_2(&self) -> usize {
        let mut image = self.image.clone();
        for _ in 0..50 {
            image = image.generate_child(&self.algo);
        }
        image.pixels.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_20.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_20 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_20 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;

    impl Display for Image {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in self.top_left.y..=self.bottom_right.y {
                for x in self.top_left.x..=self.bottom_right.x {
                    let c = if self.pixels.contains(&(x, y).into()) {
                        '#'
                    } else {
                        '.'
                    };
                    write!(f, "{c}")?;
                }
                if y < self.bottom_right.y {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }

    #[test]
    fn test_fixedbitset_from_iter() {
        let bits = [0, 2, 5];
        let bit_set = FixedBitSet::from_iter(bits);
        // LSB .. MSB
        assert_eq!(format!("{:b}", bit_set), "101001");
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_20_example.txt");
        let example = ChallengeInput::from(input);
        assert_eq!(example.image.pixels.len(), 10);

        let (algo_bits, _) = input.split_once("\n\n").unwrap();
        let fixedbitset_bits: String = (0..512)
            .map(|index| if example.algo[index] { '#' } else { '.' })
            .collect();
        assert_eq!(fixedbitset_bits, algo_bits);

        assert_eq!(example.image.bottom_right, (4, 4).into());

        assert!(
            example
                .image
                .get_new_pixel_value(&example.algo, (2, 2).into())
        );

        println!("input:\n{}", example.image);

        let child_image = example.image.generate_child(&example.algo);
        println!("output child:\n{}", child_image);

        let child_of_child_image = child_image.generate_child(&example.algo);
        println!("output child of child:\n{}", child_of_child_image);
    }

    #[test]
    fn test_example_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_20_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 35);

        let result_part2 = example.solution_part_2();
        println!("result day_20 part 2: {result_part2}");
        assert_eq!(result_part2, 3_351);

        Ok(())
    }
}
