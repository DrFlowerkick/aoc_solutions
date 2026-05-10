//!day_23.rs

use super::day_18::*;
use anyhow::Result;

impl ChallengeInput {
    fn solution_part_1_day_23(&self) -> u64 {
        let mut register = Register::new(self.actions.clone());
        while let Some(action) = register.get_action() {
            action.apply(&mut register);
        }
        register.count_mul
    }
    fn solution_part_2_day_23(&self) -> usize {
        // Note: this solution results from re-engineering the register code sequences
        // my notes for this a private, because they contain may puzzle input, but
        // I think you can guess, what the register code sequence does by reading my rust code.
        let values = self.generate_values_to_check_for_divisor();
        values
            .iter()
            .filter(|d| match **d {
                n if n % 2 == 0 => true,
                n => !(3..=n.isqrt()).step_by(2).all(|i| n % i != 0),
            })
            .count()
    }
    fn generate_values_to_check_for_divisor(&self) -> Vec<i64> {
        // reading variables from puzzle input instead of hard coding them here
        let mut start = self.extract_digit_from_action(0).expect("unexpected value");
        let factor = self.extract_digit_from_action(4).expect("unexpected value");
        let offset = self.extract_digit_from_action(5).expect("unexpected value");
        start = start * factor - offset;
        let delta_end = self.extract_digit_from_action(7).expect("unexpected value");
        let delta = self
            .extract_digit_from_action(30)
            .expect("unexpected value");
        let steps = delta_end / delta;
        (0..=steps).map(|s| start - s * delta).collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_23.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1_day_23();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, 3_025);

    let result_part2 = challenge.solution_part_2_day_23();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 915);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn text_value_generator() {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_23.txt");
        let challenge = ChallengeInput::from(input);

        let values = challenge.generate_values_to_check_for_divisor();
        assert_eq!(values[1], 105_717);
        assert_eq!(values.last(), Some(&122700));
    }
}
