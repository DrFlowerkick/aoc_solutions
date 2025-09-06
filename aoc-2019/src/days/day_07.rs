//!day_07.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;
use my_lib::my_algo_collection::RangeCombinations;

struct ChallengeInput {
    int_code_computer: IntCodeComputer,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            int_code_computer: IntCodeComputer::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        self.calc_amplifier_output()
    }
    fn solution_part_2(&self) -> i64 {
        match self.calc_amplifier_output_with_feed_back() {
            Ok(out) => out,
            Err(err) => panic!("Some error '{err}' occurred"),
        }
    }
    fn calc_amplifier_output(&self) -> i64 {
        let mut max = 0;
        for phases in RangeCombinations::new(0, 4) {
            let mut amplifiers = self.int_code_computer.clone_n_times(phases.len());
            let mut input = 0;
            for (amp, phase) in amplifiers.iter_mut().zip(phases.iter()) {
                let out = amp.run_until_finished(&[*phase, input]);
                input = out;
            }
            max = max.max(input);
        }
        max
    }
    fn calc_amplifier_output_with_feed_back(&self) -> Result<i64, String> {
        let mut max = 0;
        for phases in RangeCombinations::new(5, 9) {
            let mut amplifiers = self.int_code_computer.clone_n_times(phases.len());
            let mut input = Some(0);
            let mut last: Option<i64> = None;
            let mut first = true;
            'outer: loop {
                for (index, (amp, phase)) in amplifiers.iter_mut().zip(phases.iter()).enumerate() {
                    if let Some(inp) = input {
                        // only send phase in first round of inputs
                        let inp = if first { &[*phase, inp][..] } else { &[inp] };
                        // out of one amplifier is input of next amplifier
                        input = amp.run_int_code(inp)?;
                        // save result of last amplifier
                        if index == phases.len() - 1 {
                            last = input;
                        }
                        if input.is_none() {
                            // first halt encountered. Must be at first amplifier (index 0)!
                            assert_eq!(index, 0);
                        }
                    } else {
                        // reached halt in int code
                        if amp.run_int_code(&[])?.is_some() {
                            panic!("expected halt op code after reaching first halt")
                        }
                        // stop loop after last amplifier
                        if index == phases.len() - 1 {
                            break 'outer;
                        }
                    }
                }
                first = false;
            }
            max = max.max(last.ok_or(String::from("did not receive any output"))?);
        }
        Ok(max)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_07.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_07 part 1: {result_part1}");
    assert_eq!(result_part1, 45_730);

    let result_part2 = challenge.solution_part_2();
    println!("result day_07 part 2: {result_part2}");
    assert_eq!(result_part2, 5_406_484);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_07() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_07_example.txt");
        let solutions = [43_210, 54_321, 65_210, 139629729, 18216];

        for (index, (input, solution)) in multi_input.split("\n\n").zip(solutions).enumerate() {
            let example = ChallengeInput::from(input);

            if index < 3 {
                let result_part1 = example.solution_part_1();
                println!("result day_07 part 1: {result_part1}");
                assert_eq!(result_part1, solution);
            } else {
                let result_part2 = example.solution_part_2();
                println!("result day_07 part 2: {result_part2}");
                assert_eq!(result_part2, solution);
            }
        }

        Ok(())
    }
}
