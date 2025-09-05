//!day_05.rs

use anyhow::Result;

#[derive(Debug, Clone)]
struct ChallengeInput {
    numbers: Vec<i64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            numbers: value.split(',').filter_map(|n| n.parse().ok()).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut test_data = self.clone();
        let input = 1;
        match test_data.run_int_code(input) {
            Ok(ok) => ok,
            Err((err, index)) => panic!("Some error '{err}' occurred at index '{index}'."),
        }
    }
    fn solution_part_2(&self) -> i64 {
        let mut test_data = self.clone();
        let input = 5;
        match test_data.run_int_code(input) {
            Ok(ok) => ok,
            Err((err, index)) => panic!("Some error '{err}' occurred at index '{index}'."),
        }
    }
    fn run_int_code(&mut self, input: i64) -> Result<i64, (i64, usize)> {
        let mut index = 0;
        let mut last_out: Option<i64> = None;

        while let Some(ext_op_code) = self.numbers.get(index) {
            let op_code = ext_op_code % 100;
            let parameter_modes = ext_op_code / 100;

            match op_code {
                1 => {
                    // addition
                    let parameters = self.get_n_parameters(index, parameter_modes, 3, true)?;
                    *self
                        .numbers
                        .get_mut(parameters[2] as usize)
                        .ok_or((-4, index))? = parameters[0] + parameters[1];
                    index += 4;
                }
                2 => {
                    // multiplication
                    let parameters = self.get_n_parameters(index, parameter_modes, 3, true)?;
                    *self
                        .numbers
                        .get_mut(parameters[2] as usize)
                        .ok_or((-4, index))? = parameters[0] * parameters[1];
                    index += 4;
                }
                3 => {
                    // write input to position
                    let parameters = self.get_n_parameters(index, parameter_modes, 1, true)?;
                    *self
                        .numbers
                        .get_mut(parameters[0] as usize)
                        .ok_or((-6, index))? = input;
                    index += 2;
                }
                4 => {
                    // handle previous out
                    if let Some(out) = last_out
                        && out != 0
                    {
                        // some test resulted in error
                        return Err((out, index));
                    }
                    // return val to out
                    let parameters = self.get_n_parameters(index, parameter_modes, 1, false)?;
                    last_out = Some(parameters[0]);
                    index += 2;
                }
                5 => {
                    // jump if true
                    let parameters = self.get_n_parameters(index, parameter_modes, 2, false)?;
                    index = if parameters[0] != 0 {
                        parameters[1] as usize
                    } else {
                        index + 3
                    }
                }
                6 => {
                    // jump if false
                    let parameters = self.get_n_parameters(index, parameter_modes, 2, false)?;
                    index = if parameters[0] == 0 {
                        parameters[1] as usize
                    } else {
                        index + 3
                    }
                }
                7 => {
                    // less than
                    let parameters = self.get_n_parameters(index, parameter_modes, 3, true)?;
                    let store_val = (parameters[0] < parameters[1]) as i64;
                    *self
                        .numbers
                        .get_mut(parameters[2] as usize)
                        .ok_or((-4, index))? = store_val;
                    index += 4;
                }
                8 => {
                    // equals
                    let parameters = self.get_n_parameters(index, parameter_modes, 3, true)?;
                    let store_val = (parameters[0] == parameters[1]) as i64;
                    *self
                        .numbers
                        .get_mut(parameters[2] as usize)
                        .ok_or((-4, index))? = store_val;
                    index += 4;
                }
                99 => {
                    // immediately halt
                    break;
                }
                _ => panic!("unknown op code"),
            }
        }
        last_out.ok_or((0, index))
    }

    fn get_n_parameters(
        &self,
        index: usize,
        mut parameter_modes: i64,
        num_parameters: usize,
        last_is_write: bool,
    ) -> Result<Vec<i64>, (i64, usize)> {
        let mut parameters = vec![0; num_parameters];
        for (p_index, parameter) in parameters.iter_mut().enumerate() {
            let parameter_mode = parameter_modes % 10;
            parameter_modes /= 10;
            *parameter = match parameter_mode {
                0 => {
                    // position
                    let pos = *self.numbers.get(index + p_index + 1).ok_or((-1, index))?;
                    if pos < 0 {
                        return Err((-2, index));
                    }
                    // if writing position, just return writing position
                    if p_index == num_parameters - 1 && last_is_write {
                        pos
                    } else {
                        *self.numbers.get(pos as usize).ok_or((-1, index))?
                    }
                }
                1 => {
                    // immediate
                    // not valid for writing to position
                    if p_index == num_parameters - 1 && last_is_write {
                        return Err((-3, index));
                    }
                    *self.numbers.get(index + p_index + 1).ok_or((-1, index))?
                }
                _ => panic!("not supported parameter mode"),
            };
        }
        Ok(parameters)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_05.txt");
    let challenge = ChallengeInput::from(input);

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
    fn test_day_05_equal_less() -> Result<()> {
        // input == 8 ? with position mode
        let int_code = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(8).unwrap();
        assert_eq!(result, 1);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 0);

        // input == 8 ? with immediate mode
        let int_code = "3,3,1108,-1,8,3,4,3,99";
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(8).unwrap();
        assert_eq!(result, 1);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 0);

        // input less than 8 ? with position mode
        let int_code = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 1);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(8).unwrap();
        assert_eq!(result, 0);

        // input less than 8 ? with immediate mode
        let int_code = "3,3,1107,-1,8,3,4,3,99";
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 1);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(8).unwrap();
        assert_eq!(result, 0);

        Ok(())
    }

    #[test]
    fn test_day_05_jump() -> Result<()> {
        // jump: input == 0 ? with position mode
        let int_code = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 1);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(0).unwrap();
        assert_eq!(result, 0);

        // jump: input == 0 ? with immediate mode
        let int_code = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 1);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(0).unwrap();
        assert_eq!(result, 0);

        Ok(())
    }

    #[test]
    fn test_day_05_large_example() -> Result<()> {
        // input is below, equal or greater 8?
        let int_code = include_str!("../../../../aoc_input/aoc-2019/day_05_large_example.txt");
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(7).unwrap();
        assert_eq!(result, 999);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(8).unwrap();
        assert_eq!(result, 1000);
        let mut example = ChallengeInput::from(int_code);
        let result = example.run_int_code(9).unwrap();
        assert_eq!(result, 1001);

        Ok(())
    }
}
