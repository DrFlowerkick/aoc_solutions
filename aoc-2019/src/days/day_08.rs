//!day_08.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

struct ChallengeInput<const X: usize, const Y: usize> {
    image: Vec<MyMap2D<u64, X, Y>>,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        let mut image: Vec<MyMap2D<u64, X, Y>> = Vec::new();
        for (digit_index, digit) in value
            .chars()
            .filter_map(|d| d.to_digit(10))
            .map(|d| d as u64)
            .enumerate()
        {
            let layer_index = digit_index / (X * Y);
            let layer_mod = digit_index % (X * Y);
            if layer_mod == 0 {
                // generate new layer
                image.push(MyMap2D::default());
            }
            let x = layer_mod % X;
            let y = layer_mod / X;
            image[layer_index].set((x, y).into(), digit);
        }
        ChallengeInput { image }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&self) -> usize {
        let min_num_zero = self
            .image
            .iter()
            .min_by_key(|layer| layer.iter().filter(|(_, d)| **d == 0).count())
            .unwrap();
        min_num_zero.iter().filter(|(_, d)| **d == 1).count()
            * min_num_zero.iter().filter(|(_, d)| **d == 2).count()
    }
    fn solution_part_2(&self) -> String {
        let mut decoded_image: MyMap2D<char, X, Y> = MyMap2D::default();
        for (pos, pixel) in decoded_image.iter_mut() {
            for layer in self.image.iter() {
                match layer.get(pos) {
                    1 => {
                        *pixel = '#';
                        break;
                    }
                    0 => {
                        *pixel = ' ';
                        break;
                    }
                    _ => (),
                }
            }
        }
        format!("{decoded_image}")
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_08.txt");
    let challenge = ChallengeInput::<25, 6>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_08 part 1: {result_part1}");
    assert_eq!(result_part1, 2_016);

    let result_part2 = challenge.solution_part_2();
    println!("result day_08 part 2:\n{result_part2}");

    let solution = include_str!("../../../../aoc_input/aoc-2019/day_08_expected_part_2.txt");
    assert_eq!(result_part2, solution);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_08() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_08_example.txt");
        let (first, second) = input.split_once("\n\n").unwrap();

        let example = ChallengeInput::<3, 2>::from(first);

        let result_part1 = example.solution_part_1();
        println!("result day_08 part 1: {result_part1}");
        assert_eq!(result_part1, 1);

        let example = ChallengeInput::<2, 2>::from(second);
        let result_part2 = example.solution_part_2();
        println!("result day_08 part 2:\n{result_part2}");
        assert_eq!(result_part2, " #\n# ");

        Ok(())
    }
}
