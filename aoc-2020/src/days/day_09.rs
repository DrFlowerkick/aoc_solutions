//!day_09.rs

use anyhow::Result;

struct ChallengeInput {
    numbers: Vec<u64>,
    block_size: usize,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.lines().filter_map(|l| l.parse().ok()).collect(),
            block_size: 25,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut index = self.block_size;
        'outer: while index < self.numbers.len() {
            let next_num = self.numbers[index];
            for (i, n1) in self.numbers[index - self.block_size..index]
                .iter()
                .enumerate()
            {
                for n2 in self.numbers[index - self.block_size..index]
                    .iter()
                    .skip(i + 1)
                {
                    if next_num == *n1 + *n2 {
                        index += 1;
                        continue 'outer;
                    }
                }
            }
            return next_num;
        }
        panic!("could not find first invalid number")
    }
    fn solution_part_2(&self, invalid_num: u64) -> u64 {
        let mut index = 0;
        while index < self.numbers.len() {
            let mut off_set = 1;
            let mut check_sum = self.numbers[index];
            let mut min = check_sum;
            let mut max = check_sum;
            if check_sum == invalid_num {
                panic!("could not find contiguous set of at least two numbers");
            }
            while check_sum < invalid_num && index + off_set < self.numbers.len() {
                let next = self.numbers[index + off_set];
                check_sum += next;
                min = min.min(next);
                max = max.max(next);
                if check_sum == invalid_num {
                    return min + max;
                }
                off_set += 1;
            }
            index += 1;
        }
        panic!("could not find contiguous set of at least two numbers");
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 1_930_745_883);

    let result_part2 = challenge.solution_part_2(result_part1);
    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 268_878_261);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_09() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_09_example.txt");
        let mut example = ChallengeInput::from(input);
        example.block_size = 5;

        let result_part1 = example.solution_part_1();
        println!("result day_09 part 1: {result_part1}");
        assert_eq!(result_part1, 127);

        let result_part2 = example.solution_part_2(result_part1);
        println!("result day_09 part 2: {result_part2}");
        assert_eq!(result_part2, 62);

        Ok(())
    }
}
