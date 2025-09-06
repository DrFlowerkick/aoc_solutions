//!day_05.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IntCodeComputer {
    numbers: HashMap<i64, i64>,
    index: i64,
    relative_base: i64,
}

impl From<&str> for IntCodeComputer {
    fn from(value: &str) -> Self {
        IntCodeComputer {
            numbers: value
                .split(',')
                .enumerate()
                .filter_map(|(i, n)| n.parse().ok().map(|d| (i as i64, d)))
                .collect(),
            index: 0,
            relative_base: 0,
        }
    }
}

impl IntCodeComputer {
    fn solution_part_1(&self) -> i64 {
        let mut test_data = self.clone();
        let inputs = [1];
        test_data.run_until_finished(&inputs)
    }
    fn solution_part_2(&self) -> i64 {
        let mut test_data = self.clone();
        let inputs = [5];
        test_data.run_until_finished(&inputs)
    }
    pub fn clone_n_times(&self, n: usize) -> Vec<Self> {
        let mut clones: Vec<Self> = Vec::with_capacity(n);
        for _ in 0..n {
            clones.push(self.clone());
        }
        clones
    }
    pub fn run_until_finished(&mut self, inputs: &[i64]) -> i64 {
        let mut first_call = true;
        let mut last_out: Option<i64> = None;
        loop {
            let out = if first_call {
                first_call = false;
                self.run_int_code(inputs)
            } else {
                self.run_int_code(&[])
            };
            match out {
                Ok(Some(out)) => {
                    // handle last out.
                    if let Some(lo) = last_out
                        && lo != 0
                    {
                        // some error occurred
                        panic!(
                            "Some test error '{lo}' occurred before index '{}'.",
                            self.index
                        );
                    }
                    last_out = Some(out);
                }
                Ok(None) => {
                    // finished, handle last out
                    if let Some(out) = last_out {
                        return out;
                    }
                    panic!("no output generated");
                }
                Err(err) => panic!(
                    "Some computation error '{err}' occurred at index '{}'.",
                    self.index
                ),
            }
        }
    }
    pub fn run_int_code(&mut self, inputs: &[i64]) -> Result<Option<i64>, String> {
        // this is run like a state machine
        // it returns None if finished or Some(out), if some out command has been executed
        let mut input_index = 0;

        while let Some(ext_op_code) = self.numbers.get(&self.index) {
            let op_code = ext_op_code % 100;
            let parameter_modes = ext_op_code / 100;

            match op_code {
                1 => {
                    // addition
                    let parameters = self.get_n_parameters(parameter_modes, 3, true)?;
                    self.set(parameters[2], parameters[0] + parameters[1])?;
                    self.index += 4;
                }
                2 => {
                    // multiplication
                    let parameters = self.get_n_parameters(parameter_modes, 3, true)?;
                    self.set(parameters[2], parameters[0] * parameters[1])?;
                    self.index += 4;
                }
                3 => {
                    // write input to position
                    let parameters = self.get_n_parameters(parameter_modes, 1, true)?;
                    self.set(
                        parameters[0],
                        *inputs
                            .get(input_index)
                            .ok_or(format!("invalid input index '{input_index}'"))?,
                    )?;
                    input_index += 1;
                    self.index += 2;
                }
                4 => {
                    // return val to out
                    let parameters = self.get_n_parameters(parameter_modes, 1, false)?;
                    let out = Some(parameters[0]);
                    self.index += 2;
                    return Ok(out);
                }
                5 => {
                    // jump if true
                    let parameters = self.get_n_parameters(parameter_modes, 2, false)?;
                    self.index = if parameters[0] != 0 {
                        parameters[1]
                    } else {
                        self.index + 3
                    }
                }
                6 => {
                    // jump if false
                    let parameters = self.get_n_parameters(parameter_modes, 2, false)?;
                    self.index = if parameters[0] == 0 {
                        parameters[1]
                    } else {
                        self.index + 3
                    }
                }
                7 => {
                    // less than
                    let parameters = self.get_n_parameters(parameter_modes, 3, true)?;
                    let store_val = (parameters[0] < parameters[1]) as i64;
                    self.set(parameters[2], store_val)?;
                    self.index += 4;
                }
                8 => {
                    // equals
                    let parameters = self.get_n_parameters(parameter_modes, 3, true)?;
                    let store_val = (parameters[0] == parameters[1]) as i64;
                    self.set(parameters[2], store_val)?;
                    self.index += 4;
                }
                9 => {
                    // change relative base
                    let parameters = self.get_n_parameters(parameter_modes, 1, false)?;
                    self.relative_base += parameters[0];
                    self.index += 2;
                }
                99 => {
                    // immediately halt
                    break;
                }
                _ => panic!("unknown op code"),
            }
        }
        Ok(None)
    }

    fn get(&mut self, key: i64) -> Result<i64, String> {
        if key < 0 {
            return Err("negative index of memory".into());
        }
        // insert 0, if entry does not exist yet
        Ok(*self.numbers.entry(key).or_insert(0))
    }

    fn set(&mut self, key: i64, value: i64) -> Result<i64, String> {
        if key < 0 {
            return Err("negative index of memory".into());
        }
        // insert 0, if entry does not exist yet
        Ok(*self
            .numbers
            .entry(key)
            .and_modify(|v| *v = value)
            .or_insert(value))
    }

    fn get_n_parameters(
        &mut self,
        mut parameter_modes: i64,
        num_parameters: usize,
        last_is_write: bool,
    ) -> Result<Vec<i64>, String> {
        let mut parameters = vec![0; num_parameters];
        for (p_index, parameter) in parameters
            .iter_mut()
            .enumerate()
            .map(|(i, p)| (i as i64, p))
        {
            let parameter_mode = parameter_modes % 10;
            parameter_modes /= 10;
            *parameter = match parameter_mode {
                0 => {
                    // position
                    let pos = self.get(self.index + p_index + 1)?;
                    if pos < 0 {
                        return Err(format!("Negative position index '{pos}'"));
                    }
                    // if writing position, just return writing position
                    if p_index == (num_parameters as i64) - 1 && last_is_write {
                        pos
                    } else {
                        self.get(pos)?
                    }
                }
                1 => {
                    // immediate
                    // not valid for writing to position
                    if p_index == (num_parameters as i64) - 1 && last_is_write {
                        return Err(
                            "immediate mode not valid for writing parameter to memory".into()
                        );
                    }
                    self.get(self.index + p_index + 1)?
                }
                2 => {
                    // relative mode
                    let pos = self.get(self.index + p_index + 1)? + self.relative_base;
                    if pos < 0 {
                        return Err(format!("Negative position index '{pos}'"));
                    }
                    // if writing position, just return writing position
                    if p_index == (num_parameters as i64) - 1 && last_is_write {
                        pos
                    } else {
                        self.get(pos)?
                    }
                }
                _ => return Err("not supported parameter mode".into()),
            };
        }
        Ok(parameters)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_05.txt");
    let challenge = IntCodeComputer::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 7_692_125);

    let result_part2 = challenge.solution_part_2();
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 14_340_395);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // there is no example for day 05,which represents the challenge
    // instead there are some int code snippets we can use for testing

    #[test]
    fn test_day_05_small_examples() -> Result<()> {
        let small_examples =
            include_str!("../../../../aoc_input/aoc-2019/day_05_small_examples.txt");
        let int_code_computers: Vec<IntCodeComputer> =
            small_examples.lines().map(IntCodeComputer::from).collect();
        // input == 8 ? with position mode
        let mut example = int_code_computers[0].clone();
        let result = example.run_until_finished(&[8]);
        assert_eq!(result, 1);
        let mut example = int_code_computers[0].clone();
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 0);

        // input == 8 ? with immediate mode
        let mut example = int_code_computers[1].clone();
        let result = example.run_until_finished(&[8]);
        assert_eq!(result, 1);
        let mut example = int_code_computers[1].clone();
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 0);

        // input less than 8 ? with position mode
        let mut example = int_code_computers[2].clone();
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 1);
        let mut example = int_code_computers[2].clone();
        let result = example.run_until_finished(&[8]);
        assert_eq!(result, 0);

        // input less than 8 ? with immediate mode
        let mut example = int_code_computers[3].clone();
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 1);
        let mut example = int_code_computers[3].clone();
        let result = example.run_until_finished(&[8]);
        assert_eq!(result, 0);

        // jump: input == 0 ? with position mode
        let mut example = int_code_computers[4].clone();
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 1);
        let mut example = int_code_computers[4].clone();
        let result = example.run_until_finished(&[0]);
        assert_eq!(result, 0);

        // jump: input == 0 ? with immediate mode
        let mut example = int_code_computers[5].clone();
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 1);
        let mut example = int_code_computers[5].clone();
        let result = example.run_until_finished(&[0]);
        assert_eq!(result, 0);

        Ok(())
    }

    #[test]
    fn test_day_05_large_example() -> Result<()> {
        // input is below, equal or greater 8?
        let int_code = include_str!("../../../../aoc_input/aoc-2019/day_05_large_example.txt");
        let mut example = IntCodeComputer::from(int_code);
        let result = example.run_until_finished(&[7]);
        assert_eq!(result, 999);
        let mut example = IntCodeComputer::from(int_code);
        let result = example.run_until_finished(&[8]);
        assert_eq!(result, 1000);
        let mut example = IntCodeComputer::from(int_code);
        let result = example.run_until_finished(&[9]);
        assert_eq!(result, 1001);

        Ok(())
    }
}
